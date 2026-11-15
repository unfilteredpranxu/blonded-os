# Blonded OS — Owner-led UI design and sequential AI implementation workflow

**Status:** Binding collaboration rules for future milestones. These rules do not authorize implementation while M00 is active.

## Primary principle

**Ask the owner what they want the UI to look and behave like for EACH new screen, app, panel, module, theme, widget or component before drafting its implementation prompt.** Never invent a final visual style or substitute a general mockup for explicit user direction. Request one coherent visual brief conversationally: appearance, layout, animation and interactions. Accept screenshots and references if supplied; do not demand them. If the owner already answered in the current conversation, don't ask again.

## One-task-at-a-time loop

1. Identify exactly one bounded feature/screen/milestone from the user and current GitHub state.
2. Before any new UI work, obtain the owner's desired look, layout, controls, effects, animation and any constraints from their description or reference. For non-UI engineering tasks, don't force unnecessary visual questions.
3. ChatGPT chooses and *states explicitly* the specific AI/tool and available reasoning/effort mode for this task, based on its nature; avoid asserting unconfirmed subscription entitlements.
4. ChatGPT writes one precise, copy-paste-ready prompt: repository, actual branch/current SHA where known, target paths, owner-approved UI requirements, explicit scope/exclusions, acceptance tests, hardware limits, commits, reporting format.
5. The owner manually pastes that single prompt into the selected **local PC terminal agent: Codex CLI (`codex`) or Google Antigravity CLI (`agy`)**. The agent implements/tests within that bound, ideally on an isolated branch. Do not hand tasks to Codex Cloud, Gemini CLI, Jules, hosted Antigravity IDE or another coding agent unless the owner explicitly changes this choice.
6. The owner returns the **actual** agent summary/diff/tests/logs. ChatGPT inspects evidence (using connected GitHub when possible), marks pass/fail/unknown, diagnoses issues and supplies **only the next appropriate prompt**, which may be a fix or test. Never advance on unverified claimed success.
7. Do not give multiple future prompts, silently merge, move milestones or start redesigning subsequent screens before receiving evidence and owner approval. Keep all source and decisions in the project's GitHub repository.

## AI and effort selection guidelines — LOCAL CLI ONLY

**Approved coding agents:** **Codex CLI running on the owner's Arch PC** and **Google Antigravity CLI (AGY, command `agy`) also running on the PC**. ChatGPT remains the architect, design interviewer, prompt writer and evidence reviewer, but is not a third on-device coding agent. Do not route implementation to Codex Cloud, Gemini CLI, Jules, Code Assist or the Antigravity graphical IDE.

- **Implementation/default:** PC-local **Codex CLI**. Prefer the highest *actually offered* practical reasoning effort, normally **High** for substantial Rust/OS architecture and bug fixes; Medium where available for straightforward bounded edits. Model names and available modes must be checked in the installed CLI; never invent unavailable options.
- **Independent audit / review / alternate approach:** PC-local **Antigravity CLI (`agy`)**. Choose the strongest *available* thinking/reasoning model for difficult review, and a faster available model for read-only searching, small documentation tasks or repetitive mechanical work. Discover model choices in the installed AGY interface/configuration; never guess a model ID, assume Google AI Pro includes unlimited AGY usage, or silently change the implementation agent.
- **Workflow orchestration:** ChatGPT GPT-6 High when available for architecture, prompt writing and review. Each handoff states **AI**, **Mode**, **Task**, **Copy-paste prompt**, **Expected evidence**.
- **One writer per branch:** Codex is the default writer. AGY runs read-only cross-checks after Codex completes; it may make an isolated change only when the owner explicitly chooses AGY for that step. No simultaneous edits to the same paths.
- **Remote compute is optional infrastructure, not another coding agent:** GitHub Actions/CI can compile and test when the owner's 4 GB / HDD machine is too constrained; the instruction itself is still issued to a local CLI. Real target-hardware FPS/memory claims must be tested on the actual PC.
- **Safety:** keep filesystem/disk/boot operations off limits, preserve the active Arch session, review shell commands, do not use no-sandbox/dangerously bypassed approval flags. Protect keys/tokens and raw host logs.
- **CLI preflight:** run `command -v codex; codex --version; command -v agy; agy --help` on the host to confirm actual availability and available CLI options. Do not auto-install tools or require a GUI IDE.

## Current project facts and constraints

Repo: `unfilteredpranxu/blonded-os`. It is a Rust-first OS distribution based on Arch Linux, not a Hyprland rice and not a single application. Original custom desktop profiles, panels, dock, animated wallpapers, widgets and appearance studios have a committed requirements spec at `docs/THEME-SHELL-CONTRACT.md`. Shortcuts: Super+T = theme gallery, Super+W = wallpaper/desktop, Super+O = window effects. User wants Mac-like and feature-rich Hyprland-inspired fully distinct profiles, custom new themes later and smooth study/music multitasking. Rust-first where feasible, 500 MiB system idle **target** (not achieved), Intel HD Graphics 2000 OpenGL 3.3 / GLES 3.0 verified; no verified hardware Vulkan. Preserve functioning current Arch/Hyprland and recovery; don't remove apps without acceptance/backup.

**Milestone state:** M00 hardware/performance/recovery verification is active. Any request to design a future UI may be recorded as a specification before implementation, but do not ask Codex to implement UI or claim a bootable desktop until dependencies and stage approvals are satisfied.

## Required CLI-agent handoff report

- Repository and exact base/head commit SHA, branch and modified paths.
- Screens or components changed, why they match the user's actual visual specifications; screenshots or captured UI artifacts where obtainable.
- Build/test commands and actual pass/fail output; explicitly say when testing was impossible.
- Device or CI environment and whether tested on real Intel i3-2100/HD2000.
- Peak/idle memory, frame times, CPU, startup measurements if relevant; no invented benchmarks.
- Known regressions, risks, further testing requirements and stopping point.

## Non-goals

- Auto-generating UI when the owner's preferences have not been asked.
- Parallel, conflicting AI edits without handoff.
- Prompt dumps for multiple milestones.
- Claiming a Rust-only kernel or complete theme control of every third-party app.
