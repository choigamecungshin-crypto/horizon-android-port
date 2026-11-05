# Horizon Android launcher

This target runs Horizon's **original Rust/GPUI launcher UI** on Android. It does
not replace the launcher with Compose, WebView, or a recreated mock screen:
desktop and Android call the same `open_launcher` callback and compile the same
Home, Mods, Servers, Versions, Settings, account, import, and other launcher
views.

## Rendering

- ARM64 (`arm64-v8a`), Android API 26+.
- GPUI Mobile / wgpu on Android.
- Vulkan is preferred; OpenGL ES 3 is the fallback where supported by the
  Android graphics stack.
- Touch, keyboard/IME and Android safe-area support come from GPUI Mobile.
- Desktop builds continue to use GPUI Fast; Android dependencies are selected
  only for `target_os = "android"`.

## Build

Install Android SDK, Java 17, Rust stable, cargo-ndk, Android platform 35,
build-tools 35.0.0 and NDK 28.2.13676358.

From the repository root:

```sh
rustup target add aarch64-linux-android
cargo install cargo-ndk --locked
cargo ndk --manifest-path android/native/Cargo.toml --platform 26 -t arm64-v8a -o android/app/src/main/jniLibs build --release
gradle --no-daemon -p android assembleDebug
```

APK: `android/app/build/outputs/apk/debug/app-debug.apk`.

GitHub Actions runs the same ARM64 build and uploads the APK as
`horizon-android-arm64-debug`.

## Current boundary

This is the **launcher app target**, not the Minecraft game port. The UI and
launcher-side pages are shared with desktop, but Android-specific game process
launching, desktop-only OS integrations, document import flows, and physical
device testing still need validation or implementation. A successful APK build
does not prove every launcher action works on a phone.

Minecraft assets, jars, accounts, tokens, and game data are not bundled. Use
the launcher's existing user-owned resource/import flow when it is supported;
do not commit proprietary game assets or credentials to this repository.
