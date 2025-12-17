---
name: Web Platform Load Optimization
overview: This plan focuses on reducing initial load times and WASM bundle sizes for Makepad web apps through lazy font loading, resource optimization, progressive loading strategies, and WASM bundle optimizations.
todos:
  - id: lazy-font-js
    content: Implement lazy font loading in web.js - prioritize Latin fonts
    status: pending
  - id: font-priority-rust
    content: Add LoadPriority enum to draw/src/text/loader.rs for on-demand fonts
    status: pending
    dependencies:
      - lazy-font-js
  - id: split-deps
    content: Split dependencies into critical/deferred in platform/src/os/web/web.rs
    status: pending
    dependencies:
      - font-priority-rust
  - id: default-brotli
    content: Enable --strip and --brotli by default in cargo_makepad wasm build
    status: pending
  - id: font-subsetting
    content: Create Latin-only font subsets for initial web load
    status: pending
  - id: progressive-render
    content: Add progressive rendering - show UI with system fonts while custom fonts load
    status: pending
    dependencies:
      - lazy-font-js
      - split-deps
  - id: cache-headers
    content: Add proper Cache-Control headers for immutable assets
    status: pending
  - id: preload-hints
    content: Generate preload hints in HTML for critical resources
    status: pending
    dependencies:
      - split-deps
---

# Optimize Makepad Web Platform Loading Performance

## Problem Analysis

The current Makepad web implementation has several factors contributing to slow initial load times:

1. **Large Font Dependencies** - Chinese fonts (LXGWWenKai) are ~18MB each, emoji font is ~10MB, totaling ~46MB just for fonts
2. **Eager Resource Loading** - All dependencies are fetched synchronously before the app can render
3. **No Resource Prioritization** - Critical and non-critical resources load together
4. **Limited Compression** - Brotli compression exists but isn't enabled by default

## Implementation Strategy

### Phase 1: Font Optimization (Highest Impact)

**1.1 Implement Lazy Font Loading**

Modify [`platform/src/os/web/web.js`](platform/src/os/web/web.js) `FromWasmLoadDeps` to support prioritized loading:

- Load core fonts (IBMPlexSans ~178KB) first - required for UI rendering
- Defer loading of CJK fonts until first CJK character is encountered
- Defer emoji font loading until emoji is detected

**1.2 Create Font Subsetting for Web**

In [`widgets/fonts/`](widgets/fonts/), add a web-optimized font build:

- Create Latin-only subsets of fonts (~20-50KB vs 18MB for full CJK)
- Ship full CJK fonts only as on-demand downloads
- Consider using system fonts for CJK fallback on web

**1.3 Add Font Loading Priority System**

Extend [`draw/src/text/loader.rs`](draw/src/text/loader.rs):

- Add `LoadPriority` enum (Critical, High, Low, OnDemand)
- Fonts with `OnDemand` priority only load when character from their range is first used

### Phase 2: Progressive Loading Architecture

**2.1 Split Initial Dependency Loading**

Modify dependency loading in [`platform/src/os/web/web.rs`](platform/src/os/web/web.rs):

```rust
// Split deps into critical (render immediately) and deferred
pub struct DepPriority {
    critical: Vec<String>,  // Fonts, shaders needed for first frame
    deferred: Vec<String>,  // Large assets, CJK fonts, emoji
}
```

**2.2 Add Progressive Rendering Support**

Update [`platform/src/os/web/web.js`](platform/src/os/web/web.js) to:

- Render loading UI immediately after WASM loads
- Start rendering with placeholder/system fonts
- Progressively enhance as fonts load

**2.3 Implement Streaming WASM Compilation**

The current implementation already uses `compileStreaming`, but ensure it's working optimally:

- Verify `Content-Type: application/wasm` is set
- Use `instantiateStreaming` when possible

### Phase 3: WASM Bundle Optimization

**3.1 Enable Aggressive Optimization by Default**

Modify [`tools/cargo_makepad/src/wasm/compile.rs`](tools/cargo_makepad/src/wasm/compile.rs):

- Change default compile flags to include `--strip` 
- Enable brotli compression by default for production builds
- Consider adding `wasm-opt` pass for further size reduction

**3.2 Dead Code Elimination**

Review [`platform/src/lib.rs`](platform/src/lib.rs) and widget exports:

- Ensure unused widgets don't contribute to bundle size
- Add feature flags for optional widget groups (e.g., `feature = "designer"`)

**3.3 Add Build Size Reporting**

Extend cargo-makepad to:

- Report WASM size before/after compression
- List largest contributing modules
- Warn when fonts exceed threshold

### Phase 4: Caching and CDN Optimization

**4.1 Add Proper Cache Headers**

Update web server in [`tools/cargo_makepad/src/wasm/compile.rs`](tools/cargo_makepad/src/wasm/compile.rs):

- Add `Cache-Control` headers for immutable assets (fonts, WASM)
- Use content-hash in filenames for cache busting

**4.2 Resource Preloading Hints**

Generate `<link rel="preload">` hints in HTML for:

- Critical fonts
- WASM binary (with `as="fetch"`)

---

## Estimated Impact

| Optimization | Current | Target | Savings |

|--------------|---------|--------|---------|

| CJK fonts (deferred) | 36MB | 0 (initial) | 36MB |

| Emoji font (deferred) | 10MB | 0 (initial) | 10MB |

| Brotli compression | Off | On | ~60-70% |

| Font subsetting | 18MB/font | 50KB/font | ~99% |

**Expected Initial Load**: From ~50MB+ down to ~1-3MB compressed (excluding deferred assets)

---

## Key Files to Modify

1. [`platform/src/os/web/web.js`](platform/src/os/web/web.js) - Resource loading orchestration
2. [`platform/src/os/web/web.rs`](platform/src/os/web/web.rs) - Rust-side dependency management  
3. [`draw/src/text/loader.rs`](draw/src/text/loader.rs) - Font loading priority system
4. [`tools/cargo_makepad/src/wasm/compile.rs`](tools/cargo_makepad/src/wasm/compile.rs) - Build optimizations
5. [`widgets/src/theme_desktop_dark.rs`](widgets/src/theme_desktop_dark.rs) - Font configuration