# Android port

This branch is the Android implementation track for the existing Horizon game engine.
It is not a second launcher around a desktop executable: the game client itself must run
inside an Android activity and render to the Android surface.

## Non-negotiable compatibility goals

- Keep Horizon's existing game screens and widgets. The title screen, world list,
  create-world screen, multiplayer screen, Options, pause screen, inventory, HUD and
  in-game menus remain the existing Horizon/vanilla implementations. Android adds
  touch navigation and layout adaptation; it does not replace them with a separate
  native-looking menu.
- Keep the gameplay, world format, network protocol, resource/data packs and parity
  behavior in the shared Rust crates. Android-specific code belongs in the platform
  boundary and must not fork game rules.
- Support ARM64 Android first (the target for current Snapdragon phones); add x86_64
  emulator builds after the ARM64 path is stable.
- Expose Vulkan and OpenGL ES through wgpu. Auto mode should prefer Vulkan and fall
  back to GLES when surface/adapter/device creation fails. GLES must be a real GLES
  backend, not a desktop OpenGL compatibility claim.
- Keep both existing mod systems: content/data mods (.hmod) and native Horizon
  mods (.hnmod). Native mods are executable code and must retain the existing
  trust/permission boundary; importing a package must not silently enable it.
- Keep shader packs as a renderer capability. Horizon WGSL packs and supported
  GLSL/Iris-style packs must be tested on each Android backend; unsupported shader
  features must report a clear compatibility reason rather than silently rendering
  incorrectly.
- Do not put Mojang game jars, assets, generated game tables, APKs, signing keys,
  account tokens or prepared .local/ data in Git. The player supplies or downloads
  their own licensed game resources through the existing verified resource flow.

## Existing functionality to preserve

The shared engine already contains the original game screens, the .hmod content
mod format, the .hnmod native-mod path, and the shader pipeline. The Android port
must wire those systems into an Android lifecycle rather than reimplement them.
See [menus](menus.md), [mods](mods.md), [native mods](native-mods.md),
[shader packs](shaders.md), and [the shader API](api/shaders.md).

## Implementation stages

1. **Android process and surface:** Android activity entry point, lifecycle-safe
   winit window/surface, app-private storage paths, permissions, suspend/resume,
   back handling and clean shutdown.
2. **ARM64 build:** reproducible NDK/Rust cross-build and APK packaging in GitHub
   Actions. Keep SDK/NDK/JDK versions pinned and publish build artifacts only after
   the build and package checks pass.
3. **Rendering backends:** Vulkan and GLES adapter selection, surface recreation,
   swapchain resize/rotation, device-loss recovery, depth/format capability checks,
   and backend-specific shader validation.
4. **Original GUI and input:** preserve the existing screen tree and visual assets;
   add touch hit targets, gestures, IME/keyboard integration, controller support,
   safe-area/inset handling, and scalable UI without changing the desktop layout.
5. **Storage, accounts and resources:** Android document/storage access, private
   profile directories, secure token storage, verified resource downloads and
   import/export of worlds and packs.
6. **Mods and shaders:** install/enable/disable .hmod and .hnmod independently;
   native-code trust prompt; shader-pack import, capability checks and settings.
7. **Parity and device validation:** compare screen behavior and game harnesses with
   desktop; test ARM64 Vulkan and GLES on real Adreno and Mali devices, including
   low-memory handling, thermal behavior and lifecycle interruption.

## Current branch status

- [x] Repository forked from the upstream Horizon history.
- [x] Android NativeActivity entry point for the original launcher UI.
- [x] GPUI Mobile platform bootstrap with the same launcher view tree as desktop.
- [x] ARM64 APK workflow added to GitHub Actions.
- [ ] First successful CI APK build and package inspection.
- [ ] Install and launch on a physical ARM64 device.
- [ ] Confirm the original title/menu GUI renders from player-provided prepared resources.
- [ ] Touch gestures, text input/IME, back navigation and safe-area adaptation.
- [ ] Vulkan and GLES device validation and backend selection UI.
- [ ] Verify .hmod, .hnmod, WGSL and GLSL shader packs on Android.

This target packages the original launcher UI, not Minecraft gameplay. Home, Mods, Servers,
Versions, Settings, account and other launcher pages are compiled from the same Rust UI source
as desktop. Android game-process launching and device-level validation remain separate tasks.

A feature is not considered Android-supported merely because its shared Rust crate
compiles. It must pass an Android build and a real-device test for the relevant
backend and feature.
