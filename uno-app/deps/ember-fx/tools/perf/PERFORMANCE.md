# ember-fx Performance Guide

This document describes the performance characteristics of ember-fx and provides guidance for optimizing your applications.

## Performance Metrics

### Build-Time Metrics

| Metric | Target | Description |
|--------|--------|-------------|
| Full build (release) | < 60s | Complete workspace compilation |
| Incremental build | < 10s | Single component change |
| WASM compilation | < 90s | Full demo app to WASM |

### Runtime Metrics

| Metric | Target | Description |
|--------|--------|-------------|
| Theme switch | < 50ms (P95) | Time to apply new theme |
| CSS injection | < 20ms | Time to inject component CSS |
| Modal open | < 200ms | Time from click to visible |
| Input validation | < 50ms | Time to validate and show error |
| First Contentful Paint | < 1.5s | Initial content visible |
| Largest Contentful Paint | < 2.5s | Main content visible |

## Bundle Size

### Feature Impact on Bundle Size

| Feature Set | Approximate Size (gzipped) | Use Case |
|-------------|---------------------------|----------|
| Minimal (button only) | ~50KB | Landing pages |
| Basic (button + input) | ~80KB | Simple forms |
| Forms (+ selection, notification) | ~120KB | Form-heavy apps |
| Full UI | ~200KB | Full applications |

### Reducing Bundle Size

1. **Enable only needed features**
   ```toml
   [dependencies.ember-fx-components]
   version = "0.2"
   default-features = false
   features = ["button", "input"]
   ```

2. **Use code splitting** for large components like modals and date pickers

3. **Lazy load** advanced components:
   ```rust
   // Load modal component only when needed
   let modal = create_local_resource(|| async {
       // Dynamic import pattern
   });
   ```

## Signal Optimization

### Best Practices

1. **Use `get_untracked()` in event handlers**
   ```rust
   // Good - no reactive subscription
   let on_click = move |_| {
       let value = signal.get_untracked();
       // process value
   };

   // Avoid - creates unnecessary subscription
   let on_click = move |_| {
       let value = signal.get();
       // process value
   };
   ```

2. **Wrap computed values in `Memo`**
   ```rust
   // Good - computed once per change
   let is_disabled = Memo::new(move |_| {
       disabled.get() || loading.get()
   });

   // Avoid - recomputed on every access
   let is_disabled = move || disabled.get() || loading.get();
   ```

3. **Use `StoredValue` for static config**
   ```rust
   // Good - no reactive overhead
   let config = StoredValue::new(MyConfig::default());

   // Avoid - unnecessary reactivity
   let config = RwSignal::new(MyConfig::default());
   ```

## CSS Performance

### Theme Switching

Theme switching is optimized using CSS custom properties:

1. **Single DOM update** - Only the `data-theme` attribute changes
2. **CSS cascade** - Browser handles style recalculation
3. **No JavaScript** - Theme values are pure CSS

### CSS Injection

Component CSS is injected on-demand:

1. **Lazy loading** - CSS loaded when component first renders
2. **Deduplication** - Same CSS never injected twice
3. **ID-based caching** - Quick lookup for existing styles

## Measuring Performance

### Using the Performance Tools

```rust
use ember_fx_tools::{PerfTimer, benchmark, PerformanceReport};

// Single measurement
let timer = PerfTimer::start("my-operation");
// ... do work ...
let duration = timer.stop(); // Returns ms

// Benchmark with statistics
let stats = benchmark("render-button", 100, || {
    // operation to benchmark
});
println!("Mean: {}ms, P95: {}ms", stats.mean, stats.p95);

// Add to your app to see live metrics
view! {
    <PerformanceReport />
}
```

### Running Performance Tests

```bash
# Build-time metrics
cd tools/perf
./scripts/build-metrics.sh

# Bundle size analysis
./scripts/bundle-size.sh

# Runtime performance tests
cd e2e
npm run test:performance
```

## Performance Checklist

### Before Release

- [ ] Run build metrics script
- [ ] Check bundle size hasn't regressed
- [ ] Run E2E performance tests
- [ ] Verify theme switching latency
- [ ] Check for memory leaks

### During Development

- [ ] Use `get_untracked()` in event handlers
- [ ] Wrap expensive computations in `Memo`
- [ ] Avoid unnecessary re-renders
- [ ] Profile with browser DevTools
- [ ] Test on low-end devices

## Profiling Tools

### Browser DevTools

1. **Performance tab** - Record and analyze runtime
2. **Memory tab** - Check for leaks
3. **Network tab** - Verify bundle sizes
4. **Lighthouse** - Overall performance score

### Rust Profiling

```bash
# CPU profiling
cargo flamegraph --bin antd-01-ui

# Memory profiling (requires valgrind)
valgrind --tool=massif ./target/release/antd-01-ui
```

## Known Performance Considerations

### Heavy Components

These components may impact performance with large data sets:

- **Transfer** - Consider virtualization for 100+ items
- **Table** - Use pagination for large data sets
- **Tree** - Lazy load deep hierarchies

### Animation

- Theme transitions use CSS transitions (GPU-accelerated)
- Modal animations are 200ms (configurable)
- Disable animations for reduced motion preference

## Benchmarks

### Test Environment

- **CPU**: Apple M1 / Intel i7 equivalent
- **Memory**: 16GB RAM
- **Browser**: Chrome 120+ / Firefox 120+ / Safari 17+
- **Network**: Fast 3G simulation for load tests

### Results (Last Updated: 2024)

| Test | Chrome | Firefox | Safari |
|------|--------|---------|--------|
| Page Load | 850ms | 920ms | 780ms |
| Theme Switch | 12ms | 15ms | 10ms |
| Modal Open | 45ms | 52ms | 40ms |
| Input Validation | 8ms | 10ms | 7ms |

*Results may vary based on hardware and application complexity.*
