# Chapter 10: Deployment Strategies

## Building for Production

Always run the full quality check before deploying:

```bash
ovie test
oviec analyze src/main.ov
ovie doc check
oviec --self-check
```

Build the production binary:

```bash
oviec build --backend wasm src/main.ov -o dist/app.wasm
```

Or build distribution packages for all platforms:

```bash
oviec build-package
```

Output goes to `target/dist/`:
- `ovie-v2.3.0-linux-x64.tar.gz`
- `ovie-v2.3.0-windows-x64.zip`
- `ovie-v2.3.0-macos-arm64.tar.gz`

## Cross-Platform Compilation

Ovie supports three targets:
- `wasm` — WebAssembly (runs anywhere with a WASM runtime)
- `llvm` — Native binary via LLVM (requires LLVM feature flag)
- `interpreter` — Direct execution (development only)

For maximum portability, use WASM:

```bash
oviec build --backend wasm src/main.ov
```

## Offline-First Deployment

Ovie is designed for offline-first deployment. All dependencies are vendored — no network calls at runtime.

Verify offline capability:

```bash
# Disconnect from network, then:
ovie build
ovie test
# Both should succeed
```

## Containerization

A minimal Dockerfile for an Ovie application:

```dockerfile
FROM ubuntu:22.04
COPY linux-x64/ovie /usr/local/bin/ovie
COPY linux-x64/oviec /usr/local/bin/oviec
COPY linux-x64/std /usr/local/lib/ovie/std
COPY src/ /app/src/
COPY ovie.toml /app/
WORKDIR /app
CMD ["oviec", "run", "src/main.ov"]
```

## Monitoring and Observability

Use structured logging for production monitoring:

```ovie
use std::log::{info, error}
use std::time::{now, duration_ms}

fn handle_with_metrics(request_id: String, handler: Function) {
    mut start = now()
    info("START request=" + request_id)

    mut result = handler()

    mut elapsed = duration_ms(start, now())
    if result.is_err() {
        error("FAIL request=" + request_id + " duration=" + number_to_string(elapsed) + "ms error=" + result.unwrap_err())
    } else {
        info("OK request=" + request_id + " duration=" + number_to_string(elapsed) + "ms")
    }
}
```

## Update Strategies

Ovie uses semantic versioning. Update your `ovie.toml` dependencies:

```toml
[dependencies]
my-lib = ">=1.2.0"   # Accept any 1.x.x >= 1.2.0
```

After updating, regenerate the lock file:

```bash
ovie install
```

## Rollback Procedures

The `ovie.lock` file pins exact versions. To rollback:

1. Restore the previous `ovie.lock` from version control
2. Run `ovie install` to restore pinned versions
3. Rebuild and redeploy

Always commit `ovie.lock` to version control.
