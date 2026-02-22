#!/usr/bin/env python3
"""Koad OS utility CLI for common lane/PR/saveup workflows."""

from __future__ import annotations

import argparse
import datetime as dt
import re
import shlex
import subprocess
import sys
import tempfile
from pathlib import Path


ROLE_VALUES = ("Koad (PM)", "Gameplay", "Platform", "Experience", "User")
SAVEUP_ROLE_VALUES = ("Koad (PM)", "Gameplay", "Platform", "Experience")
KOAD_OS_BRANCH = "koad-os"


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
    if role in {"koad", "koad pm", "pm", "project manager"}:
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
    global_ledger_mode = not lane_isolated

    if global_ledger_mode and branch != KOAD_OS_BRANCH:
        raise ValueError(
            "global-ledger saveup writes tracked support artifacts under .koad/. "
            f"Run on '{KOAD_OS_BRANCH}' or use lane-isolated mode for lane contexts."
        )

    if lane_progress_sync and branch != KOAD_OS_BRANCH:
        raise ValueError(
            "lane-isolated saveup with --sync-progress-in-lane writes PROJECT_PROGRESS.md. "
            f"Run on '{KOAD_OS_BRANCH}' or omit --sync-progress-in-lane."
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

    if not args.no_progress_sync:
        dashboard = build_progress_dashboard(root)
        progress_path = root / args.progress_file
        progress_path.write_text(dashboard, encoding="utf-8")

    print(f"saveup call id: {cid}")
    print("mode: global-ledger")
    print(f"ledger: .koad/.agent-core/sessions/SAVEUP_CALLS.md")
    print(f"session log: .koad/.agent-core/sessions/LOG.md")
    if not args.no_progress_sync:
        print(f"progress: {args.progress_file}")
    return 0


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
    save.add_argument("--progress-file", default="PROJECT_PROGRESS.md")
    save.add_argument("--no-progress-sync", action="store_true")
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
