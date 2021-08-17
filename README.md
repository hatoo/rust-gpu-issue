```
> cargo build
    Blocking waiting for file lock on build directory
   Compiling builder v0.1.0 (C:\Users\hato2\Desktop\rust-gpu-issue\builder)
error: failed to run custom build command for `builder v0.1.0 (C:\Users\hato2\Desktop\rust-gpu-issue\builder)`

Caused by:
  process didn't exit successfully: `C:\Users\hato2\Desktop\rust-gpu-issue\target\debug\build\builder-8dcc5829546998ea\build-script-build` (exit code: 1)
  --- stderr
     Compiling core v0.0.0 (C:\Users\hato2\.rustup\toolchains\nightly-2021-08-10-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core)
     Compiling rustc-std-workspace-core v1.99.0 (C:\Users\hato2\.rustup\toolchains\nightly-2021-08-10-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\rustc-std-workspace-core)
     Compiling compiler_builtins v0.1.49
     Compiling libm v0.2.1
     Compiling spirv-types v0.4.0-alpha.12
     Compiling bitflags v1.3.2
     Compiling num-traits v0.2.14
     Compiling glam v0.17.3
     Compiling spirv-std v0.4.0-alpha.12
     Compiling shader v0.1.0 (C:\Users\hato2\Desktop\rust-gpu-issue\shader)
  error: internal compiler error: unexpected panic

  note: the compiler unexpectedly panicked. this is a bug.

  note: we would appreciate a bug report: https://github.com/EmbarkStudios/rust-gpu/issues/new

  note: rustc 1.56.0-nightly (ae90dcf02 2021-08-09) running on x86_64-pc-windows-msvc

  note: compiler flags: -Z unstable-options -Z codegen-backend=C:\Users\hato2\Desktop\rust-gpu-issue\target\debug\rustc_codegen_spirv.dll -Z symbol-mangling-version=legacy -C opt-level=3 -C embed-bitcode=no -C target-feature=+RayTracingKHR,+ext:SPV_KHR_ray_tracing --crate-type lib --crate-type dylib

  note: some of the compiler flags provided by cargo are hidden

  query stack during panic:
  end of query stack
  note: `rust-gpu` version 0.4.0-alpha.12

  error: could not compile `shader`
  Error: BuildFailed
```