#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$repo_root"

required_files=(
  ".koad/AGENTS.md"
  ".koad/.agent-core/ops/STARTUP_CHECKLIST.md"
  ".koad/.agent-core/IDENTITY.md"
  ".koad/.agent-core/memory/WORKING_MEMORY.md"
  ".koad/.agent-core/memory/LEARNINGS.md"
  ".koad/.agent-core/memory/USER_PREFERENCES.md"
  ".koad/.agent-core/sessions/SAVEUP_CALLS.md"
  ".koad/.agent-ops/STANDARDS_REGISTRY.md"
  ".koad/.agent-ops/CANONICAL_REQUIRED_SOURCES.md"
  ".koad/.agent-core/ops/ROLE_BOOT_PROTOCOL.md"
)

echo "repo_root: $repo_root"
echo "checking required startup files..."
for f in "${required_files[@]}"; do
  if [[ ! -f "$f" ]]; then
    echo "missing required file: $f" >&2
    exit 1
  fi
done
echo "required files: OK (${#required_files[@]})"

echo
echo "running standards freshness gate..."
python3 .koad/.agent-core/scripts/standards_sync_status.py \
  --manifest .koad/.standards/sync_manifest.json \
  --required-sources .koad/.agent-ops/CANONICAL_REQUIRED_SOURCES.md \
  --max-age-hours 24

echo
echo "role selection required."
echo "Which role should I personify in this thread: Koad (PM), Gameplay, Platform, or Experience?"

selected_role=""
while [[ -z "$selected_role" ]]; do
  read -r -p "> " role_input
  case "$role_input" in
    "Koad (PM)"|"koad"|"Koad"|"pm"|"PM"|"project manager"|"Project Manager")
      selected_role="Koad (PM)"
      ;;
    "Gameplay"|"gameplay")
      selected_role="Gameplay"
      ;;
    "Platform"|"platform")
      selected_role="Platform"
      ;;
    "Experience"|"experience")
      selected_role="Experience"
      ;;
    *)
      echo "invalid role. Enter one of: Koad (PM), Gameplay, Platform, Experience."
      ;;
  esac
done

echo
echo "selected_role: $selected_role"
echo "next: declare selected role, applicable standards IDs, risk level, and planned scope."
