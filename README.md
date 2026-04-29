# Bevy Android Template

> **Template** for building [Bevy](https://bevyengine.org) games/apps targeting
> Android, with full support for both **NativeActivity** and **GameActivity**
> backends provided by the
> [`android-activity`](https://crates.io/crates/android-activity) crate.

The template ships a working **3D shapes demo** (equivalent to Bevy's built-in
`3d/shapes` example) that runs on a real device or emulator straight out of
the box.  GitHub Actions builds debug **and** unsigned-release APKs for every
push and makes them available as downloadable artifacts.

---

## What this template provides

| Feature | Details |
|---|---|
| Bevy 3D scene | Rotating shapes (cube, sphere, torus, cylinder, capsule) with PBR lighting |
| GameActivity APK | `android/game-activity/` – better text-input & AndroidX support |
| NativeActivity APK | `android/native-activity/` – simpler, no Java/Kotlin required |
| Modular workspace | `crates/app` holds shared logic; add more crates to extend |
| Desktop binary | `crates/desktop` – run locally with `cargo run -p desktop` |
| CI / CD | `.github/workflows/android-apk.yml` builds both APKs and uploads artifacts |

---

## Repository structure

```
Cargo.toml                      ← Cargo workspace root
android-activity/               ← local android-activity library (workspace member)
crates/
  app/                          ← shared Bevy game/app logic (add your code here)
  android-game-activity/        ← Android cdylib – GameActivity entry point
  android-native-activity/      ← Android cdylib – NativeActivity entry point
  desktop/                      ← Desktop binary (Linux / macOS / Windows)
android/
  game-activity/                ← Gradle project → GameActivity APK
  native-activity/              ← Gradle project → NativeActivity APK
.github/workflows/
  ci.yml                        ← existing android-activity library CI
  android-apk.yml               ← new: builds both APKs and uploads artifacts
```

---

## Prerequisites

| Tool | Version |
|---|---|
| Rust | stable (≥ 1.85) |
| Android SDK | API 35 |
| Android NDK | 27.x |
| JDK | 17 |
| `cargo-ndk` | latest (`cargo install cargo-ndk`) |

```sh
# Install Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android

# Install cargo-ndk
cargo install cargo-ndk --locked
```

Set `ANDROID_SDK_ROOT` / `ANDROID_HOME` and `ANDROID_NDK_HOME` to point at
your SDK and NDK installations respectively.

---

## Building locally

### GameActivity APK

```sh
# 1. Build the Rust shared library
cargo ndk \
  -t arm64-v8a -t armeabi-v7a -t x86_64 \
  -o android/game-activity/app/src/main/jniLibs/ \
  -- build --release \
  -p android-game-activity

# 2. Build the APK
cd android/game-activity
./gradlew assembleDebug     # → app/build/outputs/apk/debug/
./gradlew assembleRelease   # → app/build/outputs/apk/release/  (unsigned)
```

### NativeActivity APK

```sh
# 1. Build the Rust shared library
cargo ndk \
  -t arm64-v8a -t armeabi-v7a -t x86_64 \
  -o android/native-activity/app/src/main/jniLibs/ \
  -- build --release \
  -p android-native-activity

# 2. Build the APK
cd android/native-activity
./gradlew assembleDebug     # → app/build/outputs/apk/debug/
./gradlew assembleRelease   # → app/build/outputs/apk/release/  (unsigned)
```

### Desktop (Linux / macOS / Windows)

```sh
cargo run -p desktop
```

---

## Adding more Rust crates / libraries

1. Create a new crate in `crates/` (or anywhere in the workspace):
   ```sh
   cargo new --lib crates/my-feature
   ```
2. Add it to the workspace in the root `Cargo.toml`:
   ```toml
   members = [
       # ...existing members...
       "crates/my-feature",
   ]
   ```
3. Add it as a dependency of `crates/app`:
   ```toml
   # crates/app/Cargo.toml
   [dependencies]
   my-feature = { path = "../my-feature" }
   ```
4. Call its `Plugin` (or any setup function) from `crates/app/src/lib.rs`
   inside `BevyShapesPlugin::build` (or `add_app_plugins`).

All platform launchers automatically pick up the change because they depend on
`crates/app`.

---

## GitHub Actions – building and downloading artifacts

The workflow at `.github/workflows/android-apk.yml` runs on every push and
pull request.  It:

1. Sets up JDK 17, Android SDK (API 35), NDK 27, and stable Rust.
2. Adds Android targets and installs `cargo-ndk`.
3. Caches Cargo registry, build artefacts, and Gradle caches for fast
   subsequent runs.
4. Builds the Rust cdylibs for both variants (`cargo ndk … --release`).
5. Runs `./gradlew assembleDebug` and `./gradlew assembleRelease` for each
   Gradle project.
6. Uploads four artifacts:
   - `bevy-shapes-game-activity-debug`
   - `bevy-shapes-game-activity-release-unsigned`
   - `bevy-shapes-native-activity-debug`
   - `bevy-shapes-native-activity-release-unsigned`

Download them from the **Actions → your workflow run → Artifacts** section of
the GitHub UI, or via the [GitHub CLI](https://cli.github.com/):

```sh
gh run download <run-id>
```

---

## About `android-activity`

This repository also hosts the
[`android-activity`](android-activity/README.md) crate itself – the
low-level glue layer that bridges Android's Activity lifecycle with Rust.  The
Bevy launchers in this template patch all `android-activity` Cargo dependencies
to use the local version (see `[patch.crates-io]` in the root `Cargo.toml`).

For the original `android-activity` library documentation, MSRV policy, and
API reference see [android-activity/README.md](android-activity/README.md) or
[docs.rs/android-activity](https://docs.rs/android-activity).

---

## NativeActivity vs GameActivity

| | NativeActivity | GameActivity |
|---|---|---|
| Shipped with OS | ✓ (no extra deps) | ✗ (needs `androidx.games:games-activity`) |
| Text input (IME) | limited | full |
| AndroidX features | ✗ | ✓ (AppCompatActivity) |
| Java/Kotlin code | none needed | small `MainActivity` subclass |

If you're unsure, start with **GameActivity** – it is better supported
long-term and handles soft-keyboard input correctly.

See the full comparison in
[android-activity/README.md](android-activity/README.md#should-i-use-nativeactivity-or-gameactivity).
