# Blonded OS — Owner-led UI design and sequential AI implementation workflow

**Status:** Binding collaboration rules for future milestones. These rules do not authorize implementation while M00 is active.

## Primary principle

**Ask the owner what they want the UI to look and behave like for EACH new screen, app, panel, module, theme, widget or component before drafting its implementation prompt.** Never invent a final visual style or substitute a general mockup for explicit user direction. Request one coherent visual brief conversationally: appearance, layout, animation and interactions. Accept screenshots and references if supplied; do not demand them. If the owner already answered in the current conversation, don't ask again.

## One-task-at-a-time loop

1. Identify exactly one bounded feature/screen/milestone from the user and current GitHub state.
2. Before any new UI work, obtain the owner's desired look, layout, controls, effects, animation and any constraints from their description or reference. For non-UI engineering tasks, don't force unnecessary visual questions.
3. ChatGPT chooses and *states explicitly* the specific AI/tool and available reasoning/effort mode for this task, based on its nature; avoid asserting unconfirmed subscription entitlements.
4. ChatGPT writes one precise, copy-paste-ready prompt: repository, actual branch/current SHA where known, target paths, owner-approved UI requirements, explicit scope/exclusions, acceptance tests, hardware limits, commits, reporting format.
5. The owner manually pastes that single prompt into selected Codex/Gemini tool; agent implements/tests within that bound, ideally on an isolated branch.
6. The owner returns the **actual** agent summary/diff/tests/logs. ChatGPT inspects evidence (using connected GitHub when possible), marks pass/fail/unknown, diagnoses issues and supplies **only the next appropriate prompt**, which may be a fix or test. Never advance on unverified claimed success.
7. Do not give multiple future prompts, silently merge, move milestones or start redesigning subsequent screens before receiving evidence and owner approval. Keep all source and decisions in the project's GitHub repository.

## AI and effort selection guidelines

- **ChatGPT GPT-6, High thinking:** default for system architecture, prompt writing and rigorous review; Extra High can be used for particularly difficult design/security tradeoffs if offered by the user's plan/interface.
- **Codex Cloud:** primary implementation agent for Rust/OS changes; choose the highest available substantial reasoning/effort mode (typically High). Scope each task tightly and demand proof, not claimed success.
- **Google AI Pro / Gemini:** use the strongest suitable available reasoning model for independent architecture/performance review, UI alternative analysis and verifying a Codex result; actual model and effort options may differ between Gemini/Jio products. Avoid two write agents on the same files.
- **Gemini CLI/Jules:** use only when connected and available; choose isolated branches for second implementations, tests or audits. Don't infer Gemini AI Pro guarantees unlimited use.
- **NotebookLM:** optional organized research for redacted docs and legitimate references.
- **AI choice in EVERY coding handoff**: `AI:`, `Mode:`, `Task:`, `Copy-paste prompt:`, `Expected evidence:`. For the user's zero-code workflow, never require them to write source code manually.

## Current project facts and constraints

Repo: `unfilteredpranxu/blonded-os`. It is a Rust-first OS distribution based on Arch Linux, not a Hyprland rice and not a single application. Original custom desktop profiles, panels, dock, animated wallpapers, widgets and appearance studios have a committed requirements spec at `docs/THEME-SHELL-CONTRACT.md`. Shortcuts: Super+T = theme gallery, Super+W = wallpaper/desktop, Super+O = window effects. User wants Mac-like and feature-rich Hyprland-inspired fully distinct profiles, custom new themes later and smooth study/music multitasking. Rust-first where feasible, 500 MiB system idle **target** (not achieved), Intel HD Graphics 2000 OpenGL 3.3 / GLES 3.0 verified; no verified hardware Vulkan. Preserve functioning current Arch/Hyprland and recovery; don't remove apps without acceptance/backup.

**Milestone state:** M00 hardware/performance/recovery verification is active. Any request to design a future UI may be recorded as a specification before implementation, but do not ask Codex to implement UI or claim a bootable desktop until dependencies and stage approvals are satisfied.

## Required Codex handoff report

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
