# Makepad Web-First-Class Platform Plan

## Objective

Make **web (wasm32 + browser runtime)** a first-class target in Makepad with clear feature parity, reliable tooling, and CI quality gates that match native platforms.

## Current State (from repo audit)

### What already exists
- Browser platform runtime in `platform/src/os/web/` (`web.rs`, `web_gl.rs/.js`, `web_audio.rs`, `web_midi.rs`, `web_media.rs`, `web_network.rs`, `web_socket.rs`).
- wasm bridge + message protocol in `libs/wasm_bridge/` and `platform/src/os/web/{from_wasm,to_wasm}.rs`.
- wasm build/run entry points in `tools/cargo_makepad/src/wasm/{mod.rs,compile.rs,sdk.rs}`.
- Browser serving support in `tools/web_server/` and wasm run flow in `README.md`.

### Known missing pieces (explicit TODOs)
- Web audio input path is stubbed: `platform/src/os/web/web_audio.rs` (`use_audio_inputs` TODO).
- Offscreen/depth render target support incomplete in WebGL path: `platform/src/os/web/web_gl.js` (depth target TODO).
- HTTP request body handling TODO in JS bridge decode path: `platform/src/os/web/web.js`.
- App focus hooks are defined but not fully wired: `platform/src/os/web/web.js`.
- Touch-scroll overlay path marked untested: `platform/src/os/web/web.js`.

## Definition of “First-Class Web Platform”

Web is first-class when all are true:
1. **Feature parity target is explicit** (documented supported/not-supported matrix).
2. **Build/run is one-command reliable** on clean machines.
3. **Core platform capabilities work** (rendering, input, audio out/in, networking, permissions, clipboard, file drag/drop as applicable).
4. **CI covers web continuously** (build + smoke runtime tests).
5. **Developer UX and docs are complete** (debugging, profiling, deploy guidance).

## Gap Analysis

### 1) Runtime parity gaps
- Audio input capture is missing in the Rust web backend.
- Depth target support is incomplete for non-canvas render passes.
- Some browser event plumbing is unfinished (focus/touch overlay paths).
- HTTP body encoding/decoding path needs robust binary/text handling contract.

### 2) Tooling/DevEx gaps
- wasm toolchain setup is documented, but there is no “web readiness check” command (target/tool binaries/browser constraints).
- No dedicated browser-debug guide (source maps, perf tools, common browser flags/headers).

### 3) Quality/CI gaps
- No explicit required web workflow gate in this repository path for wasm build + smoke run.
- Missing automated parity checks for common widgets/examples on web.

### 4) Product/docs gaps
- No single authoritative web support matrix and roadmap document in tree.
- No clear “known limitations” page for browser-specific differences.

## Implementation Plan

### Phase 0 — Baseline and acceptance criteria (1 week)
- Add and maintain a **web support matrix** (rendering/input/audio/network/storage/media/etc.) with status: done / partial / blocked.
- Define acceptance criteria for “web-ready” examples (splash + at least one input-heavy + one media/network example).
- Add a tracking issue checklist linked to file-level tasks.

### Phase 1 — Close core runtime TODOs (2–4 weeks)
1. Implement `use_audio_inputs` in `platform/src/os/web/web_audio.rs`:
   - microphone device selection + stream lifecycle,
   - permission integration reuse (`FromWasmCheckPermission` / `FromWasmRequestPermission` flow),
   - callback delivery and buffer format consistency with native backends.
2. Implement depth target handling in `platform/src/os/web/web_gl.js` for framebuffer passes.
3. Replace HTTP body decode placeholder in `platform/src/os/web/web.js` with content-type aware binary/text handling.
4. Wire focus/blur event dispatch and either validate or remove legacy touch overlay code path.

### Phase 2 — Web quality gates and tests (1–2 weeks)
- Add wasm build checks for selected examples in CI.
- Add smoke tests for startup + basic input + resize + audio output path.
- Add targeted tests around newly implemented paths (audio input, depth target render pass behavior, HTTP request body handling).

### Phase 3 — Tooling and DX polish (1–2 weeks)
- Add `cargo makepad wasm doctor` (or equivalent) to validate:
  - rust target installation,
  - required tool binaries,
  - browser runtime prerequisites.
- Improve error messages in wasm run/build for missing target/dependencies.
- Document local and production serving requirements (COEP/COOP/CORS/caching).

### Phase 4 — Documentation and release readiness (1 week)
- Update `README.md` web section with:
  - quickstart,
  - support matrix link,
  - known browser limitations.
- Add a “deploying Makepad web apps” guide with static hosting checklist.
- Mark web readiness per example in docs.

## Deliverables
- `WEB_FIRST_CLASS_PLAN.md` (this document).
- Web support matrix doc (new file under docs or root).
- Implemented runtime fixes for current TODO hotspots.
- CI workflow updates for wasm build/smoke checks.
- README + web deployment docs updates.

## Risks and Mitigations
- **Browser API variance** (permissions/audio/autoplay): keep capability probes and fallback paths, test Chrome/Safari/Firefox.
- **Performance regressions on wasm**: benchmark startup and frame times per release; fail CI on major regressions.
- **Threading/shared memory constraints**: explicitly document and validate COEP/COOP hosting needs.

## Recommended execution order
1. Land support matrix + acceptance criteria (Phase 0).
2. Land runtime TODO closures behind focused tests (Phase 1).
3. Add CI gates once runtime is stable (Phase 2).
4. Finish tooling/docs and announce web as first-class (Phases 3–4).
