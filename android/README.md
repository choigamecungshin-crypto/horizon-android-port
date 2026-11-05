# Horizon for Android

This packages the existing Horizon game client as an Android NativeActivity,
not as a separate launcher UI. With valid Horizon 26.3 resources present, the
existing Rust client owns the title screen, menus, settings, game screens,
rendering, mods and shaders.

## Target

- ARM64 (arm64-v8a) first; Android API 26 minimum.
- Winit 0.30 + NativeActivity.
- WGPU Vulkan and OpenGL ES backends.
- App-private files under files/horizon.
- Audio enabled through CPAL/AAudio. Desktop controller enumeration is not
  enabled in this initial mobile build.

## Build

Install Android SDK, Java 17, Rust stable, cargo-ndk, Android platform 35,
build-tools 35.0.0 and NDK 28.2.13676358.

From the repository root:

~~~sh
rustup target add aarch64-linux-android
cargo install cargo-ndk --locked
cargo ndk --platform 26 -t arm64-v8a -o android/app/src/main/jniLibs build --manifest-path android/native/Cargo.toml --release
gradle --no-daemon -p android assembleDebug
~~~

APK output: android/app/build/outputs/apk/debug/app-debug.apk.

## Resources

Minecraft assets, jars and prepared game tables are not bundled. Prepare
Horizon 26.3 resources from a Minecraft Java Edition installation licensed to
the player, then place them at:

~~~text
<app-internal-files>/horizon/.local/horizon-client/26.3/
~~~

The in-app resource importer/document picker is not implemented yet. Do not
commit prepared resources, account tokens, signing keys or APKs.

A successful cross-compile is not a device test. Vulkan, GLES, surface
recreation, lifecycle interruptions, touch/IME, mod loading and shader-pack
compatibility still need physical Adreno and Mali validation.
