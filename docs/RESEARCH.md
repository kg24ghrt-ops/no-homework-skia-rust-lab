# Research

## Sources

- Google's Skia source: https://github.com/google/skia
- Skia documentation: https://skia.org/docs/
- Rust Skia bindings: https://github.com/rust-skia/rust-skia
- Android NDK documentation: https://developer.android.com/ndk
- Android JNI guidance: https://developer.android.com/training/articles/perf-jni
- GitHub Actions documentation: https://docs.github.com/actions

## Findings

TODO: Record findings from official sources.

## Findings from rust-skia repository

### Platform Support for Android
- The rust-skia bindings provide prebuilt binaries for Android targets:
  - `aarch64-linux-android` (ARM64)
  - `x86_64-linux-android` (x86_64)
- These binaries are built for Android API Level 26 (Android 8.0, Oreo) and above.
- The build script (in skia-bindings/build.rs) attempts to download prebuilt binaries from the skia-binaries repository.
- If prebuilt binaries are not available for the target or feature set, the build script will attempt to build Skia from source, which requires LLVM, Python 3, and Ninja.

### Building for Android
- To build for Android, one must set up the Android NDK and configure the Rust target.
- Example environment variables for building for aarch64-linux-android:
  ```
  export ANDROID_NDK=/path/to/android-ndk-r26d
  export PATH=$PATH:$ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64/bin
  export CC_aarch64_linux_android=aarch64-linux-android26-clang
  export CXX_aarch64_linux_android=aarch64-linux-android26-clang++
  export AR_aarch64_linux_android=llvm-ar
  export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=aarch64-linux-android26-clang
  ```
- Then run: `cargo build -vv --target aarch64-linux-android`

### Licensing
- Skia is licensed under the BSD Free Software License.
- The rust-skia bindings are licensed under MIT.

### Dependencies
- The rust-skia crate (skia-safe) depends on the skia-bindings crate, which in turn includes the Skia source code or links to prebuilt binaries.
- For Android, the native dependencies are the Skia libraries compiled for the target ABI.

### Practicality for Android Projects
- The rust-skia bindings are actively maintained and support Android.
- The size of the native library can be significant because it includes the Skia graphics engine.
- However, using prebuilt binaries can reduce build time and complexity.

## Conclusion
Based on the research, it is feasible to use the rust-skia bindings for an Android native engine. The prebuilt binaries for Android are available, which avoids the need to compile Skia from source during every build. The licensing is permissive (BSD and MIT). The next step is to set up the project and verify the build in CI.

