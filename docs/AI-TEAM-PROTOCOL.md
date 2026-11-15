# Blonded OS — Codex + AGY team protocol

**Purpose:** Two local PC-terminal coding agents work as **one engineering team**, without repeated context, duplicated tasks or conflicting edits. This is a shared *repository-based record*, not automatic AI-to-AI memory. Every agent MUST read the tracked state before acting.

## Authoritative context, in priority order

1. `AGENTS.md`: permanent operating boundaries, project definition, safety requirements.
2. `docs/AI_TEAM_STATE.md`: **single short current checkpoint** — milestone, task owner, baseline, blockers, next allowed action. Keep <=80 lines.
3. Assigned GitHub issue + relevant design/acceptance spec (e.g. `docs/THEME-SHELL-CONTRACT.md`); read only when needed for that task.
4. `git status --short`, `git log -3 --oneline`, `git diff --stat`, relevant changed files and tests. Git history is evidence, not an excuse to reprint the whole repo.
5. `docs/ai-handoffs/<TASK-ID>-<agent>.md`: short durable handoff record **only when a coding task produces results**. Older records on demand, not on every startup.

If the working tree/branch differs from the checkpoint, trust Git evidence, report the discrepancy and request reconciliation. Never overwrite an existing agent's changes or silently discard uncommitted work.

## Responsibilities

- **ChatGPT:** gets UI intent before new visual features, chooses **Codex CLI** or **AGY CLI** and actual available effort mode, supplies **one** precise paste-ready prompt, and reviews the returned diff/tests before any next task.
- **Codex CLI on owner's Arch PC:** default implementer and tester of the one assigned change. Has write ownership only of explicitly authorized paths/branch.
- **AGY CLI on same PC:** secondary *read-only* reviewer, performance/security challenger, or explicitly assigned alternate implementer on its own isolated branch. Never edit the Codex-owned paths simultaneously.
- **Owner:** pastes prompts, provides actual terminal outcomes, chooses UI direction, approves sensitive system changes, branch merges and milestone progression.

No Codex Cloud, Gemini CLI, Jules, or extra coding agents. Do not assume account plans guarantee particular modes/models or unlimited quotas. Determine actual CLI choices from installed tools.

## Minimal-token startup (per assigned task)

**Unattended coding preference:** follow `docs/HANDS-OFF-CLI-AUTONOMY.md`. For defined code-edit/test tasks, no repeated permission requests, routine confirmations or needless questions; preserve sandbox restrictions and report blocked high-risk work instead of trying privilege bypasses. Neither agent may interpret "hands-off" as approval to modify boot, packages, private files, services or real disks.

1. Read only `AGENTS.md` and `docs/AI_TEAM_STATE.md` (~short permanent + short current).
2. Verify `git branch --show-current`, `git rev-parse --short HEAD`, `git status --short`, and whether an owner or task is already recorded.
3. Read assigned issue plus **only** task-relevant source/spec and `git diff` for changed paths. Do not recursively dump the repo, re-read entire chat, or launch broad searches.
4. Internally produce a **micro-plan** (up to 5 bullets). Work inside approved scope. For UI, owner-approved visual details must be in the prompt.
5. Run scoped tests. Write a concise report. Update checkpoint and make a separate handoff only if committed changes warrant durable detail.
6. STOP. Do not create the next assignment yourself. ChatGPT/owner decide from the evidence.

## Shared handoff format (target <=25 lines)

```md
# <Task ID> — <Codex/AGY>
State: COMPLETE | BLOCKED | FAILED | REVIEW_PENDING
Branch: <branch>
Base: <full SHA>
Head: <full SHA>
Owner: <agent>
Scope: <files/features>
Changed: <one-to-three bullets>
Tests: <command => PASS/FAIL/NOT RUN + reason>
Hardware: <actual physical host/CI, or NOT TESTED>
Risks: <specific remaining issues>
Next allowed action: <review-only, probe, or fix; not automatic milestone promotion>
```

Do **not** copy entire logs, huge diffs, images or private host output into text; record command, short result and artifact/commit links. Record **UNKNOWN** rather than inventing test results. Use issue/PR comments for complex evidence; link by issue/commit SHA.

## Collision and recovery rules

- One assigned implementer at a time. AGY can analyze while Codex is writing only if isolated in read-only mode and without competing edits.
- Before edit: check `git status` and current task owner. If dirty/another agent has ownership, STOP unless explicitly authorized to continue it.
- One branch per independent task; no force push/reset/checkout that destroys work; merge only after review and owner approval.
- The finishing writer writes a durable report and updates `docs/AI_TEAM_STATE.md` to `REVIEW_PENDING`, pointing at exact branch/commit. Then **wait** for ChatGPT's review and owner's next instruction.
- Record accepted/blocked decisions in state; don't repeat completed discovery or re-run expensive tests without a clear changed input.
- Prefer targeted `rg`, specific files, `cargo test -p <crate>` instead of broad recursive audits/builds. On 4GB/HDD hardware keep parallel Rust builds conservative; request before installing/removing packages or changing boot.
- Secret/username/IP/serial/raw host reports never go into public repo. Preserve the current functional Arch/Hyprland session.

## Context window / usage discipline

Small durable checkpoint > chat transcript. Explicit references > restating long instructions. Answer with changed facts, concise proof, blockers. No AI has shared hidden memory; it knows its teammate's work only by reading **the same Git branch, checkpoint, issues and commits**. Sync/checkout the correct branch first; unmerged documentation does **not** exist on `main` automatically.
