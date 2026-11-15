# Blonded OS — Hands-off Codex CLI / AGY CLI autonomy policy

**Owner preference:** Paste one task prompt, let the selected local PC-terminal AI implement, self-check, and leave a concise handoff. **Do not interrupt the owner for routine editing, scoped repository commands or tests.** This is not authorization for unrestricted PC access.

## Supported launches (check installed version / actual feature availability)

In the **cloned repository's root**, launch one agent at a time:

- **Codex CLI (primary):** `codex --sandbox workspace-write --ask-for-approval never`.
  - Workspace code edits, local searches, lint, formatting, Rust build/tests are expected to proceed without permission prompts **within sandbox restrictions**.
  - With approval policy `never`, out-of-sandbox commands or denied network access may fail rather than prompt. Do not loosen sandbox, disable approvals protections, or use `--yolo` to work around this.
- **AGY CLI (secondary):** `agy --mode=auto` on a version/platform offering terminal auto mode. The mode auto-approves workspace edits and normal sandboxed commands. Unsafe/irreversible actions can still request confirmation; if no owner interaction is planned, **skip the action and report BLOCKED**.
  - `agy --mode=auto` is **conditional** on a compatible Linux sandbox and AGY version; unsupported installs can fall back to interactive mode. Do not present this as guaranteed on every account.
  - Do **not** use `--dangerously-skip-permissions` or equivalent unrestricted execution flags.
- Both modes must respect the host's real Arch session and safe limits. Model name and reasoning effort are selected only from those actually present in the installed CLI; **High** for complex Codex tasks when offered.

## Standing authorization for a precisely scoped task

Within the task-designated **Blonded OS Git working tree and approved branch only**, the assigned agent can autonomously:

1. Read task-relevant source/config/docs and Git history; use short context from `AGENTS.md` + `docs/AI_TEAM_STATE.md`.
2. Create/edit files needed by the **one assigned task**, with no need to ask about each edit; run ordinary local formatting, lint, unit tests, static analysis and build commands.
3. Fix routine compile/test failures, rerun scoped checks, document known gaps and provide one complete task handoff.
4. Update compact tracked team state and handoff files only when their content is accurate and checked in the right branch.
5. Commit *task-scoped* changes if the sandbox, repository state and Git permissions actually allow it; if not, preserve a non-destructive diff and report this without escalating to unrestricted execution.

**Do not ask the owner for routine code-level design decisions** already defined in the assignment; make sensible reversible choices, record them and proceed. Before implementing a **new UI component**, the owner must have specified its desired visual style in the ChatGPT-led prompt.

## Blocked / potentially destructive operations (not covered by unattended permission)

Never autonomously do any of the following on the user's host:

- `sudo` / package installation, removal or system upgrades; modifications to system services/autostart, networking/firewall, user sessions or display manager.
- Bootloader, kernel, initramfs, partitioning, formatting, mounting real user disks, OS installer writes, real deployment.
- Destructive Git history operations (`reset --hard`, `clean -fdx`, force-push), deleting user files, overwriting uncommitted work or another agent's branch.
- Accessing/publishing private tokens, browser profiles, passwords, SSH keys, personal files outside the repo, or uploading raw system audits.
- Unreviewed network side effects, deployments, releases, publishing packages, spending money, credential changes or third-party account actions.
- Blanket disabling sandbox/permission checks. No `--dangerously-bypass-approvals-and-sandbox`, `--yolo`, `--dangerously-skip-permissions`.

When blocked, **do not wait for approval prompts mid-task**. Complete all safe parts, clearly record `BLOCKED` with the exact denied action and reason, and stop for the next owner/ChatGPT decision. The owner can separately authorize a specific system operation when available, but task prompts cannot create root-level standing authorization.

## Rules for independent two-agent collaboration

- The ChatGPT coordinator assigns **only one prompt/task at a time**, explicitly selects Codex CLI or AGY CLI, and states local reasoning mode where available.
- The writer updates `docs/AI_TEAM_STATE.md` and creates a short handoff (branch, head/base SHA, paths, tests, blockers, status); the other AI reads the same Git branch + state before starting.
- AGY audit is read-only unless explicitly assigned a separate write task and branch. No simultaneous conflicting writes or self-promotion to next milestone.
- Token use: do not read entire chat or scan full repository repeatedly. Just `AGENTS.md`, current checkpoint, assigned issue/spec, relevant files/diff and tests.
- On 4GB RAM / HDD, avoid uncontrolled parallel Cargo compiles and unbounded logs. Work is considered finished when scoped tests pass or a verifiable blocker is reported. Never claim success on unrun checks.

## User workflow

```sh
cd /path/to/blonded-os
codex --sandbox workspace-write --ask-for-approval never
# OR, in a separate invocation:
agy --mode=auto
```

Paste the one assigned prompt, then return the agent's actual Git commit/diff/test summary to ChatGPT. The CLI may require ordinary initial account login; autonomous code editing cannot bypass those product authentication or security controls.

**Milestone:** M00 remains active. Recording this policy is documentation, not authorization to build the OS, remove applications or modify the physical Arch installation.
