# Product vision

Blonded OS is a personal, mood-adaptive Linux OS distribution with a Rust-first visual environment. It should be infinitely extendable in principle through new code and declarative themes, without treating every new visual style as a separate OS installation.

## Visual freedom
- Pixelated ocean waves, ASCII/noir, cozy winter night, music, joyful and future original moods.
- Animated wallpapers that can match all window chrome, controls and panels.
- User-adjustable window *opacity*, separate background material opacity, blur, shadows, radius, border, animation and tint.
- Per-application overrides, custom placement/floating/tiling/snapping, keyboard control.
- Live palette coordination and transitions that do not crash or restart apps.
- Configurable low-power/reduced-motion fallbacks without making controls inaccessible.

## Future application tracks
- Rust browser engine evaluation (Servo is not automatically a complete daily browser).
- Discord with supported client/API surfaces only; no self-bots.
- Rust local media player, with official service authorization considered separately for Apple Music; never bypass DRM.

## Explicit non-goals of M00
Do not rewrite Linux kernel, build a compositor without GPU evidence, claim a bootable distribution, or claim <500 MiB is achieved.
