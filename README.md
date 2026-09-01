# Document Converter

Offline-first cross-platform document conversion application.

## Current status

Initial Rust domain core is scaffolded. It defines format detection, conversion requests, job states, and a pluggable engine interface. Flutter UI and concrete conversion engines will be added incrementally.

## Build

```bash
cargo test --workspace
```

## Supported formats
DOCX, PDF, XLSX, XLS, CSV, TSV, PPTX, TXT, HTML, Markdown, PNG, JPEG, WebP, GIF, BMP and TIFF are registered in the core format registry.

## Offline guarantees
The conversion core has no network client and is designed for in-process native execution. Platform bridges can package native engines directly into each application.

## Build targets
The workspace is organized for Flutter Android (APK/AAB), Windows, macOS and Linux builds. Install Flutter stable, Android SDK/NDK, and Rust targets on the build machine, then run `flutter build apk`, `flutter build appbundle`, `flutter build windows`, `flutter build macos`, or `flutter build linux` from `apps/flutter_app`.

## Core module layout
`converter_core` is split into `format`, `model`, `engine`, and `error`. The engine registry is deliberately abstract; platform FFI must expose request submission, progress events, cancellation, and result/error serialization without claiming an implementation before native linkage is added.

## FFI boundary (planned, not yet implemented)
The stable boundary will use opaque job IDs and JSON payloads: `submit_job(request_json) -> job_id`, `poll_event() -> event_json`, `cancel_job(job_id)`, and `free_string(ptr)`. This keeps Flutter independent of Rust internals and works on Android, Windows, macOS, and Linux.
