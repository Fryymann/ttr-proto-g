#!/usr/bin/env python3
"""Koad OS utility CLI for common lane/PR/saveup workflows."""

from __future__ import annotations

import argparse
import datetime as dt
import glob
import json
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path


ROLE_VALUES = ("Koad (PM)", "Gameplay", "Platform", "Experience", "User")
SAVEUP_ROLE_VALUES = ("Koad (PM)", "Gameplay", "Platform", "Experience")
KOAD_OS_BRANCH = "koad-os"
REQUIRED_PR_GATE_CHECKS = (
    "validate-pr-governance",
    "validate-koad-os-scope",
    "validate-implementation-doc",
    "validate-test-evidence",
)
REVIEW_GATE_LABELS = (
    "Coding agent self-review completed",
    "Koad git review approved",
    "Ian review approved",
)
KOAD_OS_EXACT_FILES = {
    "AGENTS.md",
    "CODEX_ROLE_PROMPTS.md",
    "ANTIGRAVITY_SPRINT_PROMPTS.md",
    "PROJECT_PROGRESS.md",
    "docs/ops/github-branch-protection.md",
    "docs/design/execution-sprint-plan.md",
    ".github/pull_request_template.md",
    ".github/workflows/pr-template-gate.yml",
    ".github/workflows/koad-os-scope-gate.yml",
    ".github/workflows/sync-koad-os-from-v1.yml",
    ".github/workflows/update-v1-dashboard.yml",
    ".github/workflows/promote-koad-os-to-v1.yml",
    ".github/workflows/implementation-doc-gate.yml",
    ".github/workflows/test-evidence-gate.yml",
}


def run(cmd: list[str], cwd: Path | None = None, check: bool = True) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        cmd,
        cwd=str(cwd) if cwd else None,
        check=check,
        text=True,
        capture_output=True,
    )


def git(args: list[str], cwd: Path | None = None, check: bool = True) -> subprocess.CompletedProcess[str]:
    return run(["git", *args], cwd=cwd, check=check)


def repo_root() -> Path:
    out = git(["rev-parse", "--show-toplevel"]).stdout.strip()
    return Path(out)


def now_utc() -> dt.datetime:
    return dt.datetime.now(dt.timezone.utc)


def call_id_now() -> str:
    return now_utc().strftime("SAVEUP-%Y%m%d-%H%M%SZ")


def normalize_role(role: str) -> str:
    role = role.strip().lower()
    if role in {"koad", "koad pm", "pm", "project manager", "koad (pm)"}:
        return "Koad"
    if role == "gameplay":
        return "Gameplay"
    if role == "platform":
        return "Platform"
    if role == "experience":
        return "Experience"
    raise ValueError(f"Unsupported role: {role}")


def slugify(value: str) -> str:
    slug = re.sub(r"[^a-zA-Z0-9._-]+", "-", value.strip())
    slug = re.sub(r"-{2,}", "-", slug).strip("-").lower()
    if not slug:
        raise ValueError("Slug cannot be empty after normalization")
    return slug


def bool_to_checkbox(value: bool) -> str:
    return "x" if value else " "


def render_file_bullets(paths: list[str]) -> str:
    if not paths:
        return "- none"
    return "\n".join(f"- `{path}`" for path in paths)


def parse_backlog_items(backlog_text: str) -> list[dict[str, str]]:
    items: list[dict[str, str]] = []
    for line in backlog_text.splitlines():
        if not line.startswith("| BL-"):
            continue
        parts = [p.strip() for p in line.strip().strip("|").split("|")]
        if len(parts) < 7:
            continue
        items.append(
            {
                "id": parts[0],
                "priority": parts[1],
                "team": parts[2],
                "state": parts[3],
                "milestone": parts[4],
                "task": parts[5],
            }
        )
    return items


def parse_focus_window(backlog_text: str) -> dict[str, str]:
    out = {"now": "n/a", "next": "n/a"}
    for line in backlog_text.splitlines():
        if line.startswith("- `Now`"):
            out["now"] = line.split(":", 1)[1].strip() if ":" in line else line.strip()
        if line.startswith("- `Next`"):
            out["next"] = line.split(":", 1)[1].strip() if ":" in line else line.strip()
    return out


def parse_sprint_status_notes(plan_text: str) -> list[str]:
    lines = plan_text.splitlines()
    start = None
    for i, line in enumerate(lines):
        if line.strip() == "## Sprint Status Notes":
            start = i + 1
            break
    if start is None:
        return []

    notes: list[str] = []
    for line in lines[start:]:
        if line.startswith("## "):
            break
        if line.startswith("- "):
            notes.append(line[2:].strip())
    return notes


def parse_packet_queue(prompts_text: str) -> list[dict[str, str]]:
    lines = prompts_text.splitlines()
    start = None
    for i, line in enumerate(lines):
        if line.strip().startswith("## Active Task Packet Queue"):
            start = i + 1
            break
    if start is None:
        return []

    rows: list[dict[str, str]] = []
    in_table = False
    for line in lines[start:]:
        if line.startswith("### "):
            break
        if line.startswith("| Packet ID |"):
            in_table = True
            continue
        if not in_table:
            continue
        if line.startswith("| ---"):
            continue
        if not line.startswith("|"):
            if rows:
                break
            continue
        parts = [p.strip() for p in line.strip().strip("|").split("|")]
        if len(parts) < 6:
            continue
        rows.append(
            {
                "packet_id": parts[0].strip("`"),
                "role": parts[1],
                "backlog_ids": parts[2],
                "status": parts[3],
                "dependency": parts[4],
                "branch": parts[5].strip("`"),
            }
        )
    return rows


def parse_active_release_branch(prompts_text: str) -> str:
    match = re.search(r"Active release base branch is `([^`]+)`", prompts_text)
    if match:
        return match.group(1)
    return "v1"


def pct(numerator: int, denominator: int) -> str:
    if denominator <= 0:
        return "0%"
    return f"{round((numerator / denominator) * 100)}%"


def build_progress_dashboard(root: Path) -> str:
    backlog_path = root / ".agents/backlog.md"
    plan_path = root / "docs/design/execution-sprint-plan.md"
    roadmap_path = root / "docs/design/game-system-roadmap.md"
    prompts_path = root / "CODEX_ROLE_PROMPTS.md"

    backlog_text = backlog_path.read_text(encoding="utf-8")
    plan_text = plan_path.read_text(encoding="utf-8")
    roadmap_text = roadmap_path.read_text(encoding="utf-8")
    prompts_text = prompts_path.read_text(encoding="utf-8")

    items = parse_backlog_items(backlog_text)
    focus = parse_focus_window(backlog_text)
    sprint_notes = parse_sprint_status_notes(plan_text)
    packets = parse_packet_queue(prompts_text)
    release_branch = parse_active_release_branch(prompts_text)

    total = len(items)
    done = sum(1 for i in items if i["state"] == "done")
    in_progress = sum(1 for i in items if i["state"] == "in_progress")
    blocked = sum(1 for i in items if i["state"] == "blocked")
    todo = sum(1 for i in items if i["state"] == "todo")
    open_count = total - done

    milestone_order = ["M1", "M2", "M3", "M4", "M5"]
    milestone_rows: list[str] = []
    m1_completed_flag = "M1 (Completed)" in roadmap_text
    for ms in milestone_order:
        ms_items = [i for i in items if i["milestone"] == ms]
        ms_total = len(ms_items)
        ms_done = sum(1 for i in ms_items if i["state"] == "done")
        ms_ip = sum(1 for i in ms_items if i["state"] == "in_progress")
        ms_todo = sum(1 for i in ms_items if i["state"] == "todo")
        ms_blocked = sum(1 for i in ms_items if i["state"] == "blocked")
        if ms_total == 0 and not (ms == "M1" and m1_completed_flag):
            continue

        status_hint = ""
        if ms == "M1" and m1_completed_flag:
            status_hint = "Completed (roadmap)"
        elif ms_total == 0:
            status_hint = "n/a"
        elif ms_done == ms_total:
            status_hint = "Complete"
        elif ms_ip > 0:
            status_hint = "In Progress"
        else:
            status_hint = "Queued"

        milestone_rows.append(
            f"| {ms} | {ms_total} | {ms_done} | {ms_ip} | {ms_todo} | {ms_blocked} | {pct(ms_done, ms_total)} | {status_hint} |"
        )

    active_items = [i for i in items if i["state"] in {"in_progress", "blocked"}]
    active_rows = [
        f"| {i['id']} | {i['team']} | {i['state']} | {i['milestone']} | {i['task']} |"
        for i in active_items
    ]
    if not active_rows:
        active_rows = ["| none | n/a | n/a | n/a | n/a |"]

    packet_rows = []
    for p in packets:
        packet_rows.append(
            f"| {p['packet_id']} | {p['role']} | {p['backlog_ids']} | {p['status']} | {p['dependency']} | {p['branch']} |"
        )
    if not packet_rows:
        packet_rows = ["| none | n/a | n/a | n/a | n/a | n/a |"]

    notes_render = sprint_notes[-8:] if sprint_notes else []
    if notes_render:
        notes_md = "\n".join(f"- {n}" for n in notes_render)
    else:
        notes_md = "- none"

    generated = now_utc().strftime("%Y-%m-%d %H:%M:%SZ")
    return f"""# Project Progress Dashboard

Auto-generated snapshot aligned to roadmap and backlog.

- Generated (UTC): {generated}
- Source files:
  - `.agents/backlog.md`
  - `docs/design/game-system-roadmap.md`
  - `docs/design/execution-sprint-plan.md`
  - `CODEX_ROLE_PROMPTS.md`

## Snapshot

- Active release branch: `{release_branch}`
- Total backlog items: `{total}`
- Done: `{done}`
- In progress: `{in_progress}`
- Blocked: `{blocked}`
- Todo: `{todo}`
- Open items remaining: `{open_count}`

## Roadmap Alignment

| Milestone | Backlog Items | Done | In Progress | Todo | Blocked | Completion | Status |
|---|---:|---:|---:|---:|---:|---:|---|
{chr(10).join(milestone_rows)}

## Active Focus Window

- Now: {focus["now"]}
- Next: {focus["next"]}

## Active Task Packet Queue

| Packet ID | Role | Backlog IDs | Status | Dependency | Suggested Branch |
|---|---|---|---|---|---|
{chr(10).join(packet_rows)}

## In-Progress / Blocked Items

| Backlog ID | Team | State | Milestone | Task |
|---|---|---|---|---|
{chr(10).join(active_rows)}

## Recent Sprint Status Notes

{notes_md}

## Update Command

```bash
bash .koad/scripts/koad progress-sync
```
"""


def cmd_progress_sync(args: argparse.Namespace) -> int:
    root = repo_root()
    content = build_progress_dashboard(root)
    out_path = root / args.output

    if args.dry_run:
        print(content)
        return 0

    out_path.write_text(content, encoding="utf-8")
    print(f"Updated {out_path}")
    return 0


def extract_md_value(content: str, label: str) -> str:
    pattern = rf"^- {re.escape(label)}: (.+)\s*$"
    match = re.search(pattern, content, flags=re.MULTILINE)
    if match:
        return match.group(1).replace("`", "").strip()
    return "n/a"


def extract_generated_utc(content: str) -> str:
    match = re.search(r"^- Generated \(UTC\): ([0-9:\- ]+Z)\s*$", content, flags=re.MULTILINE)
    if match:
        return match.group(1).strip()
    return "unknown"


def status_age_hours(generated_utc: str) -> float | None:
    if generated_utc == "unknown":
        return None
    try:
        dt_value = dt.datetime.strptime(generated_utc, "%Y-%m-%d %H:%M:%SZ").replace(tzinfo=dt.timezone.utc)
    except ValueError:
        return None
    return (now_utc() - dt_value).total_seconds() / 3600.0


def cmd_status(args: argparse.Namespace) -> int:
    root = repo_root()
    progress_path = root / args.file

    if args.refresh or not progress_path.exists():
        content = build_progress_dashboard(root)
        progress_path.write_text(content, encoding="utf-8")
    elif progress_path.exists():
        content = progress_path.read_text(encoding="utf-8")
    else:
        print(f"Missing progress file: {progress_path}", file=sys.stderr)
        return 1

    generated = extract_generated_utc(content)
    age = status_age_hours(generated)
    stale = "unknown"
    if age is not None:
        stale = "yes" if age > args.max_age_hours else "no"

    branch = extract_md_value(content, "Active release branch")
    total = extract_md_value(content, "Total backlog items")
    done = extract_md_value(content, "Done")
    in_progress = extract_md_value(content, "In progress")
    blocked = extract_md_value(content, "Blocked")
    todo = extract_md_value(content, "Todo")
    open_items = extract_md_value(content, "Open items remaining")
    now_focus = extract_md_value(content, "Now")
    next_focus = extract_md_value(content, "Next")

    if args.verbose:
        print(f"file={progress_path}")
        print(f"generated_utc={generated} age_hours={f'{age:.2f}' if age is not None else 'unknown'} stale={stale}")
        print(
            f"branch={branch} total={total} done={done} in_progress={in_progress} "
            f"blocked={blocked} todo={todo} open={open_items}"
        )
        print(f"focus_now={now_focus}")
        print(f"focus_next={next_focus}")
        return 0

    age_text = f"{age:.2f}" if age is not None else "unknown"
    print(
        f"branch={branch} total={total} done={done} in_progress={in_progress} "
        f"blocked={blocked} todo={todo} open={open_items} now=\"{now_focus}\" "
        f"next=\"{next_focus}\" generated_utc={generated} age_h={age_text} stale={stale}"
    )
    return 0


def cmd_lane_start(args: argparse.Namespace) -> int:
    root = repo_root()
    role = normalize_role(args.role)
    slug = slugify(args.slug)
    branch = args.branch or f"{args.branch_prefix}/{role}/{slug}"
    base = args.base

    if args.worktree_path:
        worktree = Path(args.worktree_path).expanduser()
    else:
        worktree = Path("/tmp") / f"ttrpg-{role.lower()}-{slug}"

    try:
        git(["show-ref", "--verify", "--quiet", f"refs/heads/{branch}"], cwd=root)
        print(f"Branch already exists: {branch}", file=sys.stderr)
        return 2
    except subprocess.CalledProcessError:
        pass

    if worktree.exists() and any(worktree.iterdir()):
        print(f"Worktree path exists and is not empty: {worktree}", file=sys.stderr)
        return 2

    if args.dry_run:
        print("Dry run:")
        print(f"  repo_root: {root}")
        print(f"  base: {base}")
        print(f"  branch: {branch}")
        print(f"  worktree: {worktree}")
        print(f"  command: git worktree add {shlex.quote(str(worktree))} -b {shlex.quote(branch)} {shlex.quote(base)}")
        return 0

    worktree.parent.mkdir(parents=True, exist_ok=True)
    git(["worktree", "add", str(worktree), "-b", branch, base], cwd=root)

    print("Lane created.")
    print(f"  branch: {branch}")
    print(f"  base: {base}")
    print(f"  worktree: {worktree}")
    print("")
    print("Onboarding evidence:")
    for cmd in (
        ["pwd"],
        ["git", "rev-parse", "--show-toplevel"],
        ["git", "rev-parse", "--abbrev-ref", "HEAD"],
        ["git", "rev-parse", "--short", "HEAD"],
        ["git", "status", "--short"],
    ):
        if cmd[0] == "pwd":
            out = run(cmd, cwd=worktree).stdout.strip()
        else:
            out = run(cmd, cwd=worktree).stdout.rstrip()
        print(f"$ {' '.join(cmd)}")
        print(out if out else "(no output)")
    return 0


def current_branch(root: Path) -> str:
    return git(["rev-parse", "--abbrev-ref", "HEAD"], cwd=root).stdout.strip()


def short_head(root: Path) -> str:
    return git(["rev-parse", "--short", "HEAD"], cwd=root).stdout.strip()


def changed_files_for_pr(root: Path, base: str, head: str) -> list[str]:
    cp = git(["diff", "--name-only", f"{base}...{head}"], cwd=root, check=False)
    if cp.returncode != 0:
        return []
    return [line.strip() for line in cp.stdout.splitlines() if line.strip()]


def build_pr_body(args: argparse.Namespace, root: Path, head: str, changed_files: list[str]) -> str:
    checks = [
        "- [x] Coding agent self-review completed",
        f"- [{bool_to_checkbox(args.koad_approved)}] Koad git review approved",
        f"- [{bool_to_checkbox(args.ian_approved)}] Ian review approved",
    ]
    body = f"""## Summary
- What changed:
  - {args.summary_what}
- Why:
  - {args.summary_why}

## Role + Task Packet
- Role: {args.role}
- Persona signature (`Koad (PM)|Gameplay|Platform|Experience|User`): {args.persona or args.role}
- Task packet id: {args.task_packet}
- Backlog item(s): {args.backlog}

## Scope + Acceptance Criteria
- In scope:
  - {args.in_scope}
- Out of scope:
  - {args.out_of_scope}
- Acceptance criteria evidence:
  - [x] AC-1 `PASS`/`FAIL` + evidence: {args.ac1}
  - [x] AC-2 `PASS`/`FAIL` + evidence: {args.ac2}

## Verification
- Tests run:
  - {args.tests_run}
- Manual checks:
  - {args.manual_checks}

## Implementation Documentation
- Primary implementation doc: {args.impl_doc}
- Changed-line coverage evidence: {args.coverage_evidence}
- Automated test updates included: {args.automated_test_updates}
- Regression tests included: {args.regression_tests}
- Negative-path tests included: {args.negative_path_tests}
- Skipped tests (must include reason or `none`): {args.skipped_tests}

## Files Changed
- Primary files:
{render_file_bullets(changed_files)}
- Out-of-scope files touched (`none` if none): {args.out_of_scope_files}

## Lane Metadata
- Worktree path: `{root}`
- Branch (head): `{head}`
- Base branch (`v1` unless explicitly overridden by Koad): `{args.base}`
- Latest commit SHA: `{short_head(root)}`
- PR dependency order / blocked-by: {args.dependencies}

## Review Gates
{checks[0]}
{checks[1]}
{checks[2]}

## Risks / Deferred Work
- Risks:
  - {args.risks}
- Deferred follow-ups:
  - {args.deferred}

Task completion gate: this task is complete only after this PR is merged.
"""
    return body


def cmd_pr_open(args: argparse.Namespace) -> int:
    root = repo_root()
    head = args.head or current_branch(root)
    changed_files = changed_files_for_pr(root, args.base, head)
    body = build_pr_body(args, root, head, changed_files)

    tmp = Path(tempfile.gettempdir()) / f"koad-pr-body-{head.replace('/', '-')}.md"
    tmp.write_text(body, encoding="utf-8")
    print(f"PR body file: {tmp}")

    if args.dry_run:
        print("")
        print(body)
        return 0

    cmd = [
        "gh",
        "pr",
        "create",
        "--base",
        args.base,
        "--head",
        head,
        "--title",
        args.title,
        "--body-file",
        str(tmp),
    ]
    if args.draft:
        cmd.append("--draft")

    cp = run(cmd, cwd=root, check=False)
    if cp.returncode != 0:
        print(cp.stderr.strip() or cp.stdout.strip(), file=sys.stderr)
        return cp.returncode

    print(cp.stdout.strip())
    return 0


def gh_json(root: Path, args: list[str]) -> dict:
    cp = run(["gh", *args], cwd=root, check=False)
    if cp.returncode != 0:
        raise ValueError(cp.stderr.strip() or cp.stdout.strip() or "gh command failed")
    try:
        return json.loads(cp.stdout)
    except json.JSONDecodeError as exc:
        raise ValueError(f"Failed to parse gh JSON output: {exc}") from exc


def is_koad_os_file(path: str) -> bool:
    return path.startswith(".koad/") or path.startswith(".agents/") or path in KOAD_OS_EXACT_FILES


def scope_violations(base_ref: str, head_ref: str, files: list[str]) -> list[str]:
    violations: list[str] = []
    if base_ref == KOAD_OS_BRANCH and head_ref == "v1":
        return violations

    if base_ref == KOAD_OS_BRANCH:
        for path in files:
            if not is_koad_os_file(path):
                violations.append(f"Out-of-scope file for koad-os PR: `{path}`")
        return violations

    if head_ref == KOAD_OS_BRANCH:
        for path in files:
            if not is_koad_os_file(path):
                violations.append(f"Out-of-scope file in koad-os sync PR: `{path}`")
        return violations

    for path in files:
        if is_koad_os_file(path):
            violations.append(f"Koad/agent support file blocked on non-koad-os PR: `{path}`")
    return violations


def required_check_blockers(checks: list[dict], required: tuple[str, ...]) -> list[str]:
    blockers: list[str] = []
    for name in required:
        matching = [c for c in checks if c.get("name") == name]
        if not matching:
            blockers.append(f"`{name}` missing")
            continue

        success = any(
            c.get("status") == "COMPLETED" and str(c.get("conclusion")).upper() in {"SUCCESS", "NEUTRAL", "SKIPPED"}
            for c in matching
        )
        if success:
            continue

        pending = any(c.get("status") != "COMPLETED" for c in matching)
        if pending:
            blockers.append(f"`{name}` pending")
            continue

        conclusions = sorted({str(c.get("conclusion") or "UNKNOWN").lower() for c in matching})
        blockers.append(f"`{name}` not passing ({', '.join(conclusions)})")
    return blockers


def missing_evidence_sections(body: str) -> list[str]:
    required_markers = (
        "## Scope + Acceptance Criteria",
        "## Verification",
        "## Files Changed",
        "## Risks / Deferred Work",
        "AC-1 `PASS`/`FAIL` + evidence:",
        "AC-2 `PASS`/`FAIL` + evidence:",
        "Out-of-scope files touched (`none` if none):",
        "Coding agent self-review completed",
        "Koad git review approved",
        "Ian review approved",
    )
    return [marker for marker in required_markers if marker not in body]


def set_checkbox_line(body: str, label: str, checked: bool) -> tuple[str, bool, bool]:
    pattern = re.compile(rf"^- \[[ xX]\] {re.escape(label)}\s*$", flags=re.MULTILINE)
    match = pattern.search(body)
    if not match:
        return body, False, False
    replacement = f"- [{'x' if checked else ' '}] {label}"
    current = match.group(0)
    if current == replacement:
        return body, False, True
    new_body = body[: match.start()] + replacement + body[match.end() :]
    return new_body, True, True


def checkbox_state(body: str, label: str) -> str:
    pattern = re.compile(rf"^- \[([ xX])\] {re.escape(label)}\s*$", flags=re.MULTILINE)
    match = pattern.search(body)
    if not match:
        return "missing"
    value = match.group(1).lower()
    return "checked" if value == "x" else "unchecked"


def unchecked_review_gate_labels(body: str, require_ian: bool) -> list[str]:
    required = ["Coding agent self-review completed", "Koad git review approved"]
    if require_ian:
        required.append("Ian review approved")

    missing_or_unchecked: list[str] = []
    for label in required:
        state = checkbox_state(body, label)
        if state != "checked":
            missing_or_unchecked.append(f"`{label}` ({state})")
    return missing_or_unchecked


def patch_pr_body(root: Path, pr_number: int, body: str) -> None:
    name_with_owner = run(["gh", "repo", "view", "--json", "nameWithOwner", "-q", ".nameWithOwner"], cwd=root).stdout.strip()
    payload_path = Path(tempfile.gettempdir()) / f"koad-pr-{pr_number}-body-patch.json"
    payload_path.write_text(json.dumps({"body": body}), encoding="utf-8")
    try:
        run(
            [
                "gh",
                "api",
                f"repos/{name_with_owner}/pulls/{pr_number}",
                "--method",
                "PATCH",
                "--input",
                str(payload_path),
                "--silent",
            ],
            cwd=root,
        )
    finally:
        payload_path.unlink(missing_ok=True)


def fetch_pr_gate_payload(root: Path, pr_number: int) -> dict:
    return gh_json(
        root,
        [
            "pr",
            "view",
            str(pr_number),
            "--json",
            "number,title,url,state,isDraft,baseRefName,headRefName,mergeable,mergeStateStatus,reviewDecision,statusCheckRollup,files,body",
        ],
    )


def evaluate_pr_gate_payload(payload: dict) -> dict:
    files = [f.get("path", "") for f in (payload.get("files") or [])]
    checks = payload.get("statusCheckRollup") or []
    body = payload.get("body") or ""

    merge_blockers: list[str] = []
    state = str(payload.get("state"))
    is_draft = bool(payload.get("isDraft"))
    mergeable = str(payload.get("mergeable"))
    merge_state_status = str(payload.get("mergeStateStatus"))
    if state != "OPEN":
        merge_blockers.append(f"PR state is `{state}` (must be OPEN)")
    if is_draft:
        merge_blockers.append("PR is draft")
    if mergeable != "MERGEABLE":
        merge_blockers.append(f"mergeable=`{mergeable}`")
    if merge_state_status in {"BLOCKED", "DIRTY", "UNKNOWN"}:
        merge_blockers.append(f"mergeStateStatus=`{merge_state_status}`")

    check_blockers = required_check_blockers(list(checks), REQUIRED_PR_GATE_CHECKS)
    scope_blockers = scope_violations(str(payload.get("baseRefName", "")), str(payload.get("headRefName", "")), files)
    evidence_blockers = missing_evidence_sections(body)

    blockers: list[str] = []
    blockers.extend(merge_blockers)
    blockers.extend(check_blockers)
    blockers.extend(scope_blockers)
    blockers.extend([f"Missing PR evidence marker: `{m}`" for m in evidence_blockers])
    verdict = "approve" if not blockers else "changes requested"

    findings: list[tuple[str, str]] = []
    for item in merge_blockers + check_blockers + scope_blockers:
        findings.append(("high", item))
    for marker in evidence_blockers:
        findings.append(("medium", f"Missing PR evidence marker: `{marker}`"))

    return {
        "files": files,
        "checks": checks,
        "body": body,
        "merge_blockers": merge_blockers,
        "check_blockers": check_blockers,
        "scope_blockers": scope_blockers,
        "evidence_blockers": evidence_blockers,
        "blockers": blockers,
        "findings": findings,
        "verdict": verdict,
    }


def blocker_is_transient_for_watch(message: str) -> bool:
    if message.startswith("`") and (" pending" in message or " missing" in message):
        return True
    return message in {
        "mergeable=`UNKNOWN`",
        "mergeStateStatus=`BLOCKED`",
        "mergeStateStatus=`UNKNOWN`",
    }


def cmd_pr_gate(args: argparse.Namespace) -> int:
    root = repo_root()
    if args.watch_timeout_seconds < 1:
        raise ValueError("--watch-timeout-seconds must be >= 1")
    if args.poll_seconds < 1:
        raise ValueError("--poll-seconds must be >= 1")

    start = time.monotonic()
    deadline = start + float(args.watch_timeout_seconds)
    payload = fetch_pr_gate_payload(root, args.pr)
    evaluation = evaluate_pr_gate_payload(payload)
    watch_used = False

    while args.watch and evaluation["verdict"] != "approve":
        blockers = list(evaluation["blockers"])
        if blockers and not all(blocker_is_transient_for_watch(b) for b in blockers):
            break

        now = time.monotonic()
        if now >= deadline:
            break

        watch_used = True
        remaining = int(deadline - now)
        blocker_text = "; ".join(blockers) if blockers else "pending state reconciliation"
        print(
            f"[pr-gate watch] waiting ({remaining}s remaining): {blocker_text}",
            file=sys.stderr,
        )
        time.sleep(max(1.0, float(args.poll_seconds)))
        payload = fetch_pr_gate_payload(root, args.pr)
        evaluation = evaluate_pr_gate_payload(payload)

    if watch_used:
        elapsed = int(time.monotonic() - start)
        print(f"[pr-gate watch] final evaluation after {elapsed}s.", file=sys.stderr)

    findings = list(evaluation["findings"])
    verdict = str(evaluation["verdict"])

    print("Findings:")
    if findings:
        for idx, (severity, text) in enumerate(findings, start=1):
            print(f"{idx}. [{severity}] {text}")
    else:
        print("1. [none] No blocking findings.")

    print("\nMerge gate verdict:")
    print(f"- {verdict}")

    blockers = list(evaluation["blockers"])
    if blockers:
        print("\nRequired fixes:")
        for idx, item in enumerate(blockers, start=1):
            print(f"{idx}. {item}")
    else:
        print("\nRequired fixes:")
        print("1. none")

    if args.apply_koad_approved:
        if verdict != "approve":
            print("\nCannot auto-apply Koad approval checkbox because gate verdict is not approve.", file=sys.stderr)
            return 2

        body = str(evaluation["body"])
        updated_body, changed, found = set_checkbox_line(body, "Koad git review approved", checked=True)
        if not found:
            print("\nCannot find `Koad git review approved` checkbox line in PR body.", file=sys.stderr)
            return 2

        if args.dry_run:
            if changed:
                print("\nDry run: would update `Koad git review approved` to checked.")
            else:
                print("\nDry run: `Koad git review approved` already checked.")
            if args.comment:
                print("Dry run: would post Koad PM approval comment.")
            return 0

        if changed:
            patch_pr_body(root, int(payload["number"]), updated_body)
            print("\nUpdated PR body: checked `Koad git review approved`.")
        else:
            print("\nPR body already had `Koad git review approved` checked.")

        if args.comment:
            comment = (
                "Koad PM gate review: approved. "
                "Required checks pass (validate-pr-governance, validate-koad-os-scope), "
                "scope is policy-compliant, and mergeability is clean."
            )
            run(["gh", "pr", "comment", str(payload["number"]), "--body", comment], cwd=root)
            print("Posted Koad PM approval comment.")

    return 0


def choose_merge_strategy(root: Path, requested: str) -> str:
    if requested in {"squash", "rebase", "merge"}:
        return requested

    if requested != "auto":
        raise ValueError(f"Unsupported merge strategy: {requested}")

    repo_info = gh_json(
        root,
        [
            "repo",
            "view",
            "--json",
            "squashMergeAllowed,rebaseMergeAllowed,mergeCommitAllowed",
        ],
    )

    if repo_info.get("squashMergeAllowed"):
        return "squash"
    if repo_info.get("rebaseMergeAllowed"):
        return "rebase"
    if repo_info.get("mergeCommitAllowed"):
        return "merge"
    raise ValueError("Repository does not allow squash/rebase/merge strategies.")


def merge_flag(strategy: str) -> str:
    if strategy == "squash":
        return "--squash"
    if strategy == "rebase":
        return "--rebase"
    if strategy == "merge":
        return "--merge"
    raise ValueError(f"Unsupported merge strategy: {strategy}")


def cmd_pr_finish(args: argparse.Namespace) -> int:
    root = repo_root()
    if args.watch_timeout_seconds < 1:
        raise ValueError("--watch-timeout-seconds must be >= 1")
    if args.poll_seconds < 1:
        raise ValueError("--poll-seconds must be >= 1")

    start = time.monotonic()
    deadline = start + float(args.watch_timeout_seconds)
    payload = fetch_pr_gate_payload(root, args.pr)
    evaluation = evaluate_pr_gate_payload(payload)
    watch_used = False

    while args.watch and evaluation["verdict"] != "approve":
        blockers = list(evaluation["blockers"])
        if blockers and not all(blocker_is_transient_for_watch(b) for b in blockers):
            break

        now = time.monotonic()
        if now >= deadline:
            break

        watch_used = True
        remaining = int(deadline - now)
        blocker_text = "; ".join(blockers) if blockers else "pending state reconciliation"
        print(
            f"[pr-finish watch] waiting ({remaining}s remaining): {blocker_text}",
            file=sys.stderr,
        )
        time.sleep(max(1.0, float(args.poll_seconds)))
        payload = fetch_pr_gate_payload(root, args.pr)
        evaluation = evaluate_pr_gate_payload(payload)

    if watch_used:
        elapsed = int(time.monotonic() - start)
        print(f"[pr-finish watch] final evaluation after {elapsed}s.", file=sys.stderr)

    blockers = list(evaluation["blockers"])
    if evaluation["verdict"] != "approve":
        print("Cannot finish PR: merge gates not satisfied.", file=sys.stderr)
        for item in blockers:
            print(f"- {item}", file=sys.stderr)
        return 2

    body = str(evaluation["body"])
    review_gate_blockers = unchecked_review_gate_labels(body, require_ian=not args.skip_ian_checkbox)
    if review_gate_blockers:
        print("Cannot finish PR: review gate checkboxes are not fully approved.", file=sys.stderr)
        for item in review_gate_blockers:
            print(f"- {item}", file=sys.stderr)
        return 2

    strategy = choose_merge_strategy(root, args.strategy)
    flag = merge_flag(strategy)
    pr_url = str(payload.get("url") or f"#{args.pr}")

    if args.dry_run:
        print(f"Dry run: PR {pr_url} is merge-ready.")
        print(f"Dry run: would run `gh pr merge {args.pr} {flag}`")
        if args.delete_branch:
            print("Dry run: would include `--delete-branch`.")
        return 0

    cmd = ["gh", "pr", "merge", str(args.pr), flag]
    if args.delete_branch:
        cmd.append("--delete-branch")
    run(cmd, cwd=root)

    merged = gh_json(root, ["pr", "view", str(args.pr), "--json", "state,mergedAt,mergeCommit,url"])
    if str(merged.get("state")) != "MERGED":
        print(f"Merge command completed but PR state is `{merged.get('state')}`.", file=sys.stderr)
        return 1

    oid = (merged.get("mergeCommit") or {}).get("oid", "unknown")
    print(f"Merged {merged.get('url')} with strategy `{strategy}`.")
    print(f"merge_commit={oid} merged_at={merged.get('mergedAt')}")
    return 0


def append_line(path: Path, line: str) -> None:
    with path.open("a", encoding="utf-8") as f:
        f.write(line + "\n")


def slug_for_path(value: str) -> str:
    slug = re.sub(r"[^a-zA-Z0-9._-]+", "-", value.strip())
    slug = re.sub(r"-{2,}", "-", slug).strip("-").lower()
    return slug or "unknown-context"


def default_lane_saveup_path(root: Path, context_ref: str) -> Path:
    lane_dir = root / ".koad/.agent-core/sessions/lane-saveups"
    lane_dir.mkdir(parents=True, exist_ok=True)
    return lane_dir / f"{slug_for_path(context_ref)}.md"


def append_lane_saveup_entry(
    path: Path,
    cid: str,
    role: str,
    context_ref: str,
    scope: str,
    result: str,
    new_learnings: int,
    duplicates_skipped: int,
    notes: str,
    objective: str,
    actions: list[str],
    artifacts: list[str],
    risks: list[str],
) -> None:
    if not path.exists():
        header = [
            "# Lane Saveup Journal",
            "",
            "Append-only lane-scoped saveup records for conflict-resistant team-role continuity.",
            "",
            "## Fields",
            "- call_id",
            "- role",
            "- context_ref",
            "- scope",
            "- result",
            "- new_learnings",
            "- duplicates_skipped",
            "- notes",
        ]
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text("\n".join(header) + "\n", encoding="utf-8")

    timestamp = now_utc().strftime("%Y-%m-%d %H:%M:%SZ")
    lines = [
        "",
        f"## {cid}",
        f"- Timestamp (UTC): {timestamp}",
        f"- Role: `{role}`",
        f"- Context ref: `{context_ref}`",
        f"- Scope: {scope}",
        f"- Result: {result}",
        f"- New learnings: {new_learnings}",
        f"- Duplicates skipped: {duplicates_skipped}",
        f"- Notes: {notes}",
        f"- Objective: {objective}",
        "- Actions:",
    ]
    if actions:
        lines.extend([f"  - {a}" for a in actions])
    else:
        lines.append("  - none")

    lines.append("- Artifacts:")
    if artifacts:
        lines.extend([f"  - `{a}`" for a in artifacts])
    else:
        lines.append("  - none")

    lines.append("- Risks/Unknowns:")
    if risks:
        lines.extend([f"  - {r}" for r in risks])
    else:
        lines.append("  - none")

    with path.open("a", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")


def append_saveup_call_row(root: Path, row: str) -> None:
    calls = root / ".koad/.agent-core/sessions/SAVEUP_CALLS.md"
    append_line(calls, row)


def append_session_log(
    root: Path,
    title: str,
    role: str,
    context_ref: str,
    objective: str,
    actions: list[str],
    artifacts: list[str],
    risks: list[str],
) -> None:
    log = root / ".koad/.agent-core/sessions/LOG.md"
    today = now_utc().strftime("%Y-%m-%d")

    lines = [
        "",
        f"## {today} - {title}",
        f"- Role: `{role}`",
        f"- Context ref: `{context_ref}`",
        f"- Objective: {objective}",
        "- Actions:",
    ]
    if actions:
        lines.extend([f"  - {a}" for a in actions])
    else:
        lines.append("  - none")

    lines.append("- Artifacts:")
    if artifacts:
        lines.extend([f"  - `{a}`" for a in artifacts])
    else:
        lines.append("  - none")

    lines.append("- Risks/Unknowns:")
    if risks:
        lines.extend([f"  - {r}" for r in risks])
    else:
        lines.append("  - none")

    with log.open("a", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")


def cmd_saveup(args: argparse.Namespace) -> int:
    root = repo_root()
    branch = current_branch(root)
    cid = call_id_now()
    lane_isolated = args.lane_isolated or (
        (not args.global_ledger)
        and args.role != "Koad (PM)"
        and args.context_ref.startswith("lane/")
    )
    lane_progress_sync = lane_isolated and (not args.no_progress_sync) and args.sync_progress_in_lane
    global_progress_sync = (not lane_isolated) and args.sync_progress and (not args.no_progress_sync)
    global_ledger_mode = not lane_isolated

    if global_ledger_mode and branch not in {KOAD_OS_BRANCH, "gemini"}:
        raise ValueError(
            "global-ledger saveup writes tracked support artifacts under .koad/. "
            f"Run on '{KOAD_OS_BRANCH}', 'gemini', or use lane-isolated mode for lane contexts."
        )

    if lane_progress_sync and branch not in {KOAD_OS_BRANCH, "gemini"}:
        raise ValueError(
            "lane-isolated saveup with --sync-progress-in-lane writes PROJECT_PROGRESS.md. "
            f"Run on '{KOAD_OS_BRANCH}', 'gemini', or omit --sync-progress-in-lane."
        )

    row = (
        f"| {cid} | {args.role} | {args.context_ref} | {args.scope} | {args.result} | "
        f"{args.new_learnings} | {args.duplicates_skipped} | {args.notes} |"
    )

    if args.dry_run:
        print("Dry run:")
        print(f"mode: {'lane-isolated' if lane_isolated else 'global-ledger'}")
        print(row)
        return 0

    if lane_isolated:
        lane_path = root / args.lane_file if args.lane_file else default_lane_saveup_path(root, args.context_ref)
        append_lane_saveup_entry(
            path=lane_path,
            cid=cid,
            role=args.role,
            context_ref=args.context_ref,
            scope=args.scope,
            result=args.result,
            new_learnings=args.new_learnings,
            duplicates_skipped=args.duplicates_skipped,
            notes=args.notes,
            objective=args.objective,
            actions=args.action or [],
            artifacts=args.artifact or [],
            risks=args.risk or [],
        )

        should_sync_progress = (not args.no_progress_sync) and args.sync_progress_in_lane
        if should_sync_progress:
            dashboard = build_progress_dashboard(root)
            progress_path = root / args.progress_file
            progress_path.write_text(dashboard, encoding="utf-8")

        print(f"saveup call id: {cid}")
        print("mode: lane-isolated")
        print(f"lane journal: {lane_path}")
        print("global ledger: skipped (.koad/.agent-core/sessions/SAVEUP_CALLS.md)")
        print("global session log: skipped (.koad/.agent-core/sessions/LOG.md)")
        if should_sync_progress:
            print(f"progress: {args.progress_file}")
        else:
            print("progress: skipped (lane-isolated default)")
        return 0

    append_saveup_call_row(root, row)
    append_session_log(
        root=root,
        title=args.session_title or args.scope,
        role=args.role,
        context_ref=args.context_ref,
        objective=args.objective,
        actions=args.action or [],
        artifacts=args.artifact or [],
        risks=args.risk or [],
    )

    if global_progress_sync:
        dashboard = build_progress_dashboard(root)
        progress_path = root / args.progress_file
        progress_path.write_text(dashboard, encoding="utf-8")

    print(f"saveup call id: {cid}")
    print("mode: global-ledger")
    print(f"ledger: .koad/.agent-core/sessions/SAVEUP_CALLS.md")
    print(f"session log: .koad/.agent-core/sessions/LOG.md")
    if global_progress_sync:
        print(f"progress: {args.progress_file}")
    else:
        print("progress: skipped (global default; use --sync-progress to refresh)")
    return 0


def cmd_context(args: argparse.Namespace) -> int:
    root = repo_root()
    role = normalize_role(args.role)
    
    # Files to read
    core_files = [
        ".koad/.agent-core/IDENTITY.md",
        ".koad/.agent-core/MISSION.md",
        ".koad/.agent-core/memory/WORKING_MEMORY.md",
        ".koad/.agent-core/memory/LEARNINGS.md",
        ".koad/.agent-core/memory/USER_PREFERENCES.md",
    ]
    
    role_file = ""
    if role == "Koad":
        role_file = ".agents/roles/project-manager.md"
    elif role == "Gameplay":
        role_file = ".agents/roles/gameplay-lead.md"
    elif role == "Platform":
        role_file = ".agents/roles/platform-lead.md"
    elif role == "Experience":
        role_file = ".agents/roles/experience-lead.md"
        
    backlog_file = ".agents/backlog.md"
    risk_file = ".agents/risk-register.md"

    output = [f"# Boot Context for {role}"]
    output.append(f"Timestamp (UTC): {now_utc().isoformat()}")
    
    # Read Core
    output.append("\n# Core Memory")
    for path in core_files:
        p = root / path
        if p.exists():
            output.append(f"\n--- {path} ---\n{p.read_text(encoding='utf-8').strip()}")
        else:
            output.append(f"\n--- {path} (MISSING) ---")
            
    # Read Role
    output.append(f"\n# Role Context: {role}")
    p = root / role_file
    if p.exists():
         output.append(f"\n--- {role_file} ---\n{p.read_text(encoding='utf-8').strip()}")
    else:
        output.append(f"\n--- {role_file} (MISSING) ---")

    # Read Backlog Focus
    p = root / backlog_file
    if p.exists():
        text = p.read_text(encoding="utf-8")
        focus = parse_focus_window(text)
        output.append(f"\n# Active Focus\n- Now: {focus['now']}\n- Next: {focus['next']}")
    else:
        output.append("\n# Active Focus (Backlog Missing)")
    
    # Read Active Risks
    p = root / risk_file
    if p.exists():
        text = p.read_text(encoding="utf-8")
        output.append("\n# Active Risks (Top 3)")
        risk_lines = []
        in_table = False
        count = 0
        for line in text.splitlines():
            if line.startswith("| ID |"):
                in_table = True
                continue
            if not in_table or line.startswith("| ---"):
                continue
            if not line.startswith("|"):
                break
            # Just capture the top few for context brevity
            risk_lines.append(line)
            count += 1
            if count >= 3:
                break
        if risk_lines:
             output.append("\n".join(risk_lines))
        else:
             output.append("(No active risks found)")

    # Dump output
    print("\n".join(output))
    return 0


def parse_lane_saveup_content(text: str) -> list[dict]:
    # Extract entries starting with ## SAVEUP-...
    entries = []
    current_entry = {}
    lines = text.splitlines()
    
    for i, line in enumerate(lines):
        if line.startswith("## SAVEUP-"):
            if current_entry:
                entries.append(current_entry)
            current_entry = {"call_id": line.strip("## ").strip()}
            continue
        
        if not current_entry:
            continue
            
        if line.startswith("- Role:"):
            current_entry["role"] = line.split(":", 1)[1].replace("`", "").strip()
        elif line.startswith("- Context ref:"):
            current_entry["context_ref"] = line.split(":", 1)[1].replace("`", "").strip()
        elif line.startswith("- Scope:"):
            current_entry["scope"] = line.split(":", 1)[1].strip()
        elif line.startswith("- Result:"):
            current_entry["result"] = line.split(":", 1)[1].strip()
        elif line.startswith("- New learnings:"):
            try:
                current_entry["new_learnings"] = int(line.split(":", 1)[1].strip())
            except ValueError:
                current_entry["new_learnings"] = 0
        elif line.startswith("- Duplicates skipped:"):
             try:
                current_entry["duplicates_skipped"] = int(line.split(":", 1)[1].strip())
             except ValueError:
                current_entry["duplicates_skipped"] = 0
        elif line.startswith("- Notes:"):
            current_entry["notes"] = line.split(":", 1)[1].strip()
        elif line.startswith("- Objective:"):
            current_entry["objective"] = line.split(":", 1)[1].strip()
            
        # Collect lists (Action, Artifacts, Risks) - simplistic parsing for summary log
        # For full log reconstruction we'd need more robust parsing, but for global log synthesis 
        # we mainly need the summary fields + objective + actions list.
        if line.startswith("- Actions:"):
            current_entry["actions"] = []
            j = i + 1
            while j < len(lines) and lines[j].strip().startswith("- "):
                 # Sub-bullets are usually indented, but we look for dashed lines that aren't headers
                 # Actually in the format: "  - Action..."
                 if lines[j].strip().startswith("- Artifacts:") or lines[j].strip().startswith("- Risks/Unknowns:") or lines[j].startswith("## "):
                     break
                 current_entry["actions"].append(lines[j].strip("- ").strip())
                 j += 1

    if current_entry:
        entries.append(current_entry)
    return entries


def cmd_saveup_reconcile(args: argparse.Namespace) -> int:
    root = repo_root()
    if current_branch(root) != KOAD_OS_BRANCH and not args.force:
        print(f"Error: Reconcile must run on {KOAD_OS_BRANCH} (or use --force).", file=sys.stderr)
        return 1
        
    lane_dir = root / ".koad/.agent-core/sessions/lane-saveups"
    archive_dir = lane_dir / "archive"
    archive_dir.mkdir(exist_ok=True)
    
    files = list(lane_dir.glob("*.md"))
    if not files:
        print("No lane saveup journals found to reconcile.")
        return 0
        
    reconciled_count = 0
    entries_processed = 0
    
    for p in files:
        if p.name.lower() == "readme.md":
            continue
            
        content = p.read_text(encoding="utf-8")
        entries = parse_lane_saveup_content(content)
        
        for entry in entries:
            # Append to SAVEUP_CALLS.md
            row = (
                f"| {entry.get('call_id')} | {entry.get('role')} | {entry.get('context_ref')} | "
                f"{entry.get('scope')} | {entry.get('result')} | {entry.get('new_learnings', 0)} | "
                f"{entry.get('duplicates_skipped', 0)} | {entry.get('notes')} |"
            )
            append_saveup_call_row(root, row)
            
            # Append to LOG.md
            append_session_log(
                root,
                title=f"Reconciled: {entry.get('scope')}",
                role=entry.get('role', 'unknown'),
                context_ref=entry.get('context_ref', 'unknown'),
                objective=entry.get('objective', 'Lane saveup reconciliation'),
                actions=entry.get('actions', []),
                artifacts=[str(p.relative_to(root))],
                risks=["Reconciled from lane journal"]
            )
            entries_processed += 1

        if not args.dry_run:
            shutil.move(str(p), str(archive_dir / p.name))
        reconciled_count += 1
        
    if args.dry_run:
        print(f"Dry run: Would reconcile {reconciled_count} files containing {entries_processed} entries.")
    else:
        print(f"Reconciled {reconciled_count} files containing {entries_processed} entries.")
        print(f"Archived to {archive_dir}")
        
    return 0


def parse_iso_utc(value: str) -> dt.datetime:
    if value.endswith("Z"):
        value = value[:-1] + "+00:00"
    try:
        dt_val = dt.datetime.fromisoformat(value)
    except ValueError:
        return dt.datetime.now(dt.timezone.utc)
    if dt_val.tzinfo is None:
        dt_val = dt_val.replace(tzinfo=dt.timezone.utc)
    return dt_val.astimezone(dt.timezone.utc)


def extract_required_paths(path: Path) -> list[str]:
    if not path.exists():
        return []
    text = path.read_text(encoding="utf-8", errors="replace")
    candidates = re.findall(r"`([^`]+)`", text)
    out = []
    for item in candidates:
        if item.startswith(".") and "/" in item:
            out.append(item)
    seen = set()
    unique = []
    for item in out:
        if item not in seen:
            seen.add(item)
            unique.append(item)
    return unique


def cmd_standards_check(args: argparse.Namespace) -> int:
    root = repo_root()
    manifest_path = root / args.manifest
    required_sources_path = root / args.required_sources

    status = "FRESH"
    synced_at = None
    age_hours = None
    missing = []

    if not manifest_path.exists():
        status = "MISSING_MANIFEST"
    else:
        try:
            manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
            synced_at_raw = manifest.get("synced_at")
            if not synced_at_raw:
                status = "INVALID_MANIFEST"
            else:
                synced_at = parse_iso_utc(synced_at_raw)
                age_hours = (now_utc() - synced_at).total_seconds() / 3600.0
                if age_hours > args.max_age_hours:
                    status = "STALE"
        except Exception:
            status = "INVALID_MANIFEST"

    required_paths = extract_required_paths(required_sources_path)
    for rel in required_paths:
        if not (root / rel).exists():
            missing.append(rel)

    if missing and status == "FRESH":
        status = "MISSING_REQUIRED_SOURCES"

    print(f"manifest: {manifest_path}")
    print(f"required_sources: {required_sources_path}")
    print(f"synced_at: {synced_at.isoformat().replace('+00:00', 'Z') if synced_at else 'unknown'}")
    print(f"age_hours: {age_hours:.2f}" if age_hours is not None else "age_hours: unknown")
    print(f"max_age_hours: {args.max_age_hours:.2f}")
    print(f"required_paths_checked: {len(required_paths)}")
    print(f"missing_required_paths: {len(missing)}")
    if missing:
        for path in missing:
            print(f"missing: {path}")
    print(f"status: {status}")
    
    return 1 if status != "FRESH" else 0


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="koad", description="Koad OS workflow helper CLI")
    sub = p.add_subparsers(dest="cmd", required=True)

    lane = sub.add_parser("lane-start", help="Create lane branch + worktree with onboarding output")
    lane.add_argument("--role", required=True, help="Role name (Gameplay|Platform|Experience|Koad)")
    lane.add_argument("--slug", required=True, help="Task slug for branch/worktree naming")
    lane.add_argument("--base", default="v1", help="Base branch/commit (default: v1)")
    lane.add_argument("--branch", help="Explicit branch name override")
    lane.add_argument("--branch-prefix", default="lane", help="Branch prefix (default: lane)")
    lane.add_argument("--worktree-path", help="Explicit worktree path")
    lane.add_argument("--dry-run", action="store_true")
    lane.set_defaults(func=cmd_lane_start)

    pr = sub.add_parser("pr-open", help="Create PR from current branch with templated body")
    pr.add_argument("--title", required=True, help="PR title")
    pr.add_argument("--role", required=True, choices=ROLE_VALUES)
    pr.add_argument("--persona", choices=ROLE_VALUES)
    pr.add_argument("--task-packet", default="n/a")
    pr.add_argument("--backlog", default="n/a")
    pr.add_argument("--base", default="v1")
    pr.add_argument("--head", help="Head branch (default: current branch)")
    pr.add_argument("--summary-what", default="TBD")
    pr.add_argument("--summary-why", default="TBD")
    pr.add_argument("--in-scope", default="TBD")
    pr.add_argument("--out-of-scope", default="none")
    pr.add_argument("--ac1", default="TBD")
    pr.add_argument("--ac2", default="TBD")
    pr.add_argument("--tests-run", default="not run")
    pr.add_argument("--manual-checks", default="not run")
    pr.add_argument("--impl-doc", default="`docs/implementation/n-a-support-pr.md`")
    pr.add_argument("--coverage-evidence", default="n/a (support-scope PR)")
    pr.add_argument("--automated-test-updates", default="n/a (support-scope PR)")
    pr.add_argument("--regression-tests", default="n/a (support-scope PR)")
    pr.add_argument("--negative-path-tests", default="n/a (support-scope PR)")
    pr.add_argument("--skipped-tests", default="none")
    pr.add_argument("--out-of-scope-files", default="none")
    pr.add_argument("--dependencies", default="none")
    pr.add_argument("--risks", default="none")
    pr.add_argument("--deferred", default="none")
    pr.add_argument("--koad-approved", action="store_true")
    pr.add_argument("--ian-approved", action="store_true")
    pr.add_argument("--user-approved", dest="ian_approved", action="store_true", help=argparse.SUPPRESS)
    pr.add_argument("--draft", action="store_true")
    pr.add_argument("--dry-run", action="store_true")
    pr.set_defaults(func=cmd_pr_open)

    gate = sub.add_parser("pr-gate", help="Run PM PR gate checks; optionally apply Koad approval checkbox")
    gate.add_argument("--pr", required=True, type=int, help="PR number to review")
    gate.add_argument(
        "--apply-koad-approved",
        action="store_true",
        help="If verdict is approve, mark `Koad git review approved` checkbox in PR body",
    )
    gate.add_argument(
        "--comment",
        action="store_true",
        help="When used with --apply-koad-approved, post a Koad PM approval comment",
    )
    gate.add_argument(
        "--watch",
        action="store_true",
        help="Poll until required checks settle or timeout before final verdict",
    )
    gate.add_argument(
        "--watch-timeout-seconds",
        type=int,
        default=600,
        help="Max watch duration in seconds (default: 600)",
    )
    gate.add_argument(
        "--poll-seconds",
        type=int,
        default=8,
        help="Polling interval in seconds when --watch is enabled (default: 8)",
    )
    gate.add_argument("--dry-run", action="store_true", help="Do not write PR body/comment changes")
    gate.set_defaults(func=cmd_pr_gate)

    finish = sub.add_parser("pr-finish", help="Verify merge gates/review boxes and merge PR with allowed strategy")
    finish.add_argument("--pr", required=True, type=int, help="PR number to merge")
    finish.add_argument(
        "--strategy",
        choices=("auto", "squash", "rebase", "merge"),
        default="auto",
        help="Merge strategy (default: auto, prefer squash then rebase then merge)",
    )
    finish.add_argument(
        "--watch",
        action="store_true",
        help="Poll until required checks settle or timeout before merge evaluation",
    )
    finish.add_argument(
        "--watch-timeout-seconds",
        type=int,
        default=600,
        help="Max watch duration in seconds (default: 600)",
    )
    finish.add_argument(
        "--poll-seconds",
        type=int,
        default=8,
        help="Polling interval in seconds when --watch is enabled (default: 8)",
    )
    finish.add_argument(
        "--skip-ian-checkbox",
        action="store_true",
        help="Allow merge even if `Ian review approved` checkbox is unchecked",
    )
    finish.add_argument("--delete-branch", action="store_true", help="Request branch deletion after merge")
    finish.add_argument("--dry-run", action="store_true")
    finish.set_defaults(func=cmd_pr_finish)

    save = sub.add_parser("saveup", help="Append role-aware saveup ledger/session entries")
    save.add_argument("--role", required=True, choices=SAVEUP_ROLE_VALUES)
    save.add_argument("--context-ref", required=True, help="Task packet id, lane branch, or n/a")
    save.add_argument("--scope", required=True, help="Short saveup scope label")
    save.add_argument("--result", default="completed", choices=("completed", "partial", "blocked"))
    save.add_argument("--new-learnings", type=int, default=0)
    save.add_argument("--duplicates-skipped", type=int, default=0)
    save.add_argument("--notes", required=True)
    save.add_argument("--session-title", help="Optional session title override")
    save.add_argument("--objective", default="saveup continuity checkpoint")
    save.add_argument("--action", action="append", help="Action line (repeatable)")
    save.add_argument("--artifact", action="append", help="Artifact path line (repeatable)")
    save.add_argument("--risk", action="append", help="Risk/unknown line (repeatable)")
    save.add_argument(
        "--lane-isolated",
        action="store_true",
        help="Write saveup entry to lane journal instead of shared global ledger",
    )
    save.add_argument(
        "--global-ledger",
        action="store_true",
        help="Force global SAVEUP_CALLS/LOG writes even for lane contexts (requires koad-os branch)",
    )
    save.add_argument(
        "--lane-file",
        help="Explicit lane journal path (repo-relative) when using lane-isolated mode",
    )
    save.add_argument(
        "--sync-progress-in-lane",
        action="store_true",
        help="Allow PROJECT_PROGRESS.md refresh in lane-isolated mode (requires koad-os branch)",
    )
    save.add_argument(
        "--sync-progress",
        action="store_true",
        help="Refresh PROJECT_PROGRESS.md in global-ledger mode (disabled by default)",
    )
    save.add_argument("--progress-file", default="PROJECT_PROGRESS.md")
    save.add_argument(
        "--no-progress-sync",
        action="store_true",
        help="Force-disable PROJECT_PROGRESS.md refresh even if sync flags are set",
    )
    save.add_argument("--dry-run", action="store_true")
    save.set_defaults(func=cmd_saveup)

    progress = sub.add_parser("progress-sync", help="Generate root project progress dashboard markdown")
    progress.add_argument("--output", default="PROJECT_PROGRESS.md")
    progress.add_argument("--dry-run", action="store_true")
    progress.set_defaults(func=cmd_progress_sync)

    status = sub.add_parser("status", help="Print compact project progress summary from dashboard")
    status.add_argument("--file", default="PROJECT_PROGRESS.md")
    status.add_argument("--refresh", action="store_true", help="Refresh dashboard before reporting")
    status.add_argument("--max-age-hours", type=float, default=24.0)
    status.add_argument("--verbose", action="store_true")
    status.set_defaults(func=cmd_status)
    
    context = sub.add_parser("context", help="Dump combined core + role memory context")
    context.add_argument("--role", required=True, choices=ROLE_VALUES)
    context.set_defaults(func=cmd_context)
    
    reconcile = sub.add_parser("saveup-reconcile", help="Merge lane saveups into global ledger")
    reconcile.add_argument("--dry-run", action="store_true")
    reconcile.add_argument("--force", action="store_true", help="Run even if not on koad-os branch")
    reconcile.set_defaults(func=cmd_saveup_reconcile)
    
    standards = sub.add_parser("standards-check", help="Verify standards freshness and source presence")
    standards.add_argument("--manifest", default=".koad/.standards/sync_manifest.json")
    standards.add_argument("--required-sources", default=".koad/.agent-ops/CANONICAL_REQUIRED_SOURCES.md")
    standards.add_argument("--max-age-hours", type=float, default=24.0)
    standards.set_defaults(func=cmd_standards_check)

    return p


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    try:
        return int(args.func(args))
    except subprocess.CalledProcessError as exc:
        err = exc.stderr.strip() if exc.stderr else str(exc)
        print(err, file=sys.stderr)
        return exc.returncode or 1
    except ValueError as exc:
        print(str(exc), file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
