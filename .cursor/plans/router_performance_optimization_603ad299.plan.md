---
name: Router Performance Optimization
overview: Enhance router performance by optimizing route pattern matching, parameter lookups, widget management, event handling, and navigation operations through data structure improvements and lazy loading strategies.
todos:
  - id: hashmap-params
    content: Replace Vec with HashMap for RouteParams and RouteQuery for O(1) lookups
    status: pending
  - id: optimize-pattern-matching
    content: Optimize RouteRegistry::resolve_path with early exits and caching
    status: pending
  - id: optimize-history
    content: Add reverse index for navigation history to optimize pop_to operations
    status: pending
  - id: lazy-widget-loading
    content: Implement lazy loading of route widgets instead of creating all upfront
    status: pending
  - id: optimize-event-handling
    content: Only process events for active route widget, add flag for inactive event processing
    status: pending
  - id: optimize-widget-finding
    content: Cache active route widget reference and optimize find_widgets/uid_to_widget
    status: pending
  - id: optimize-nested-routing
    content: Cache child router pattern matches and optimize nested route resolution
    status: pending
  - id: url-parsing-cache
    content: Add LRU cache for URL parsing to avoid redundant parsing
    status: pending
---

# Router Performance Enhancement Plan

## Overview

This plan addresses performance bottlenecks in the Makepad router by optimizing data structures, reducing unnecessary operations, and implementing lazy loading strategies. The enhancements focus on route matching, parameter lookups, widget management, and navigation operations.

## Current Performance Issues

1. **Route Pattern Matching**: O(n) linear search through all patterns in `RouteRegistry::resolve_path`
2. **Route Parameters**: Linear search through Vec for parameter lookups (O(n))
3. **Route Query Parameters**: Linear search through Vec for query lookups (O(n))
4. **Widget Management**: All route widgets created upfront, consuming memory even when inactive
5. **Event Handling**: Events processed for ALL route widgets, not just active ones
6. **Navigation History**: Operations like `pop_to` use O(n) `rposition` search
7. **Nested Route Resolution**: Iterates through all child routers sequentially
8. **Widget Finding**: Linear search through all route widgets in `find_widgets` and `uid_to_widget`

## Implementation Plan

### Phase 1: Data Structure Optimizations

#### 1.1 Optimize Route Parameter Storage

**File**: `libs/router/src/route.rs`

- Replace `Vec<(LiveId, LiveId)>` in `RouteParams` with `HashMap<LiveId, LiveId>` for O(1) lookups
- Replace `Vec<(String, String)>` in `RouteQuery` with `HashMap<String, String>` for O(1) lookups
- Maintain backward compatibility by implementing `SerBin`, `DeBin`, `SerRon`, `DeRon` for HashMap-based structures
- Update `get_param`, `add`, `get` methods to use HashMap operations

**Impact**: Reduces parameter/query lookups from O(n) to O(1)

#### 1.2 Optimize Route Pattern Matching

**File**: `libs/router/src/router.rs`

- Implement a trie-based route matcher for faster pattern matching
- Alternative: Use a priority-ordered Vec but add early-exit optimizations:
- Cache static segment matches
- Pre-filter patterns by first segment before full matching
- Use a small HashMap for exact static path lookups
- Keep priority-based matching but optimize the iteration

**Impact**: Reduces pattern matching from O(n) to O(log n) or better with early exits

#### 1.3 Optimize Navigation History Operations

**File**: `libs/router/src/navigation.rs`

- For `pop_to`: Maintain a reverse index `HashMap<LiveId, Vec<usize>>` mapping route IDs to stack positions
- Update index on push/pop operations
- Use index for O(1) lookup instead of O(n) `rposition`

**Impact**: Reduces `pop_to` from O(n) to O(1) with O(1) maintenance overhead

### Phase 2: Widget Management Optimizations

#### 2.1 Implement Lazy Route Widget Loading

**File**: `libs/router/src/widget.rs`

- Change `after_apply` to only create widgets for the initial route
- Implement `ensure_route_widget` to lazily create widgets on first navigation
- Add widget cleanup strategy:
- Option A: Keep all widgets (current behavior, but optimize events)
- Option B: Unload inactive widgets after a timeout or memory pressure
- Option C: Keep last N routes in memory, unload older ones

**Impact**: Reduces initial memory footprint and startup time

#### 2.2 Optimize Event Handling

**File**: `libs/router/src/widget.rs` (handle_event method)

- Only process events for the active route widget by default
- Add a flag `process_inactive_events: bool` for cases where inactive routes need events
- For inactive routes, only process events if explicitly needed (e.g., for buttons that trigger navigation)

**Impact**: Reduces event processing overhead by ~90% for apps with many routes

#### 2.3 Optimize Widget Finding

**File**: `libs/router/src/widget.rs` (find_widgets, uid_to_widget methods)

- Cache active route widget reference to avoid iteration
- For `uid_to_widget`: Check active route first, then fall back to iteration
- Consider maintaining a `WidgetUid -> WidgetRef` cache if widget UIDs are stable

**Impact**: Reduces widget lookup time for active route from O(n) to O(1)

### Phase 3: Navigation Optimizations

#### 3.1 Optimize Nested Route Resolution

**File**: `libs/router/src/widget/nested.rs`

- Cache child router pattern matches
- Pre-compute pattern priorities and sort child routers by priority
- Early-exit when exact match found

**Impact**: Reduces nested route resolution time, especially with many child routers

#### 3.2 Optimize URL Parsing

**File**: `libs/router/src/url.rs`

- Cache parsed URLs when the same URL is navigated to multiple times
- Use a small LRU cache (e.g., last 10-20 URLs)
- Only parse when URL string changes

**Impact**: Reduces redundant URL parsing operations

#### 3.3 Optimize Route Change Callbacks

**File**: `libs/router/src/widget.rs`

- Batch callback invocations
- Consider using a single callback registry with priorities
- Avoid cloning routes unnecessarily in callback loops

**Impact**: Reduces overhead of route change notifications

### Phase 4: Memory and Allocation Optimizations

#### 4.1 Reduce Route Cloning

**Files**: Multiple files

- Use references where possible instead of cloning routes
- Only clone when necessary (e.g., for callbacks that need owned data)
- Use `Cow<Route>` for conditional ownership

**Impact**: Reduces allocations during navigation

#### 4.2 Optimize String Allocations

**File**: `libs/router/src/url.rs`, `libs/router/src/route.rs`

- Use string interning for common route paths
- Reuse string buffers where possible
- Consider using `SmallString` or `CompactString` for small paths

**Impact**: Reduces memory allocations for route paths and URLs

### Phase 5: Advanced Optimizations (Optional)

#### 5.1 Route Pattern Compilation

**File**: `libs/router/src/route.rs`

- Pre-compile route patterns into optimized matchers
- Use finite automata or regex-like compilation for complex patterns
- Cache compiled patterns

**Impact**: Faster pattern matching for complex routes

#### 5.2 Transition Optimization

**File**: `libs/router/src/widget/transitions.rs`

- Only render transition effects when transitions are active
- Use dirty tracking to avoid unnecessary redraws
- Optimize draw list operations during transitions

**Impact**: Smoother transitions with less CPU usage

## Implementation Order

1. **High Priority** (Immediate impact):

- Phase 1.1: HashMap for params/queries
- Phase 2.2: Optimize event handling
- Phase 2.3: Optimize widget finding

2. **Medium Priority** (Significant improvement):

- Phase 1.2: Optimize pattern matching
- Phase 1.3: Optimize navigation history
- Phase 2.1: Lazy widget loading

3. **Low Priority** (Nice to have):

- Phase 3: Navigation optimizations
- Phase 4: Memory optimizations
- Phase 5: Advanced optimizations

## Testing Strategy

- Add benchmarks for route matching, parameter lookups, and navigation operations
- Test with large numbers of routes (100+) to verify scalability
- Ensure backward compatibility with existing route definitions
- Verify memory usage improvements with widget lazy loading

## Files to Modify

- `libs/router/src/route.rs` - Parameter/query storage, pattern compilation
- `libs/router/src/router.rs` - Route registry, pattern matching
- `libs/router/src/navigation.rs` - History operations
- `libs/router/src/widget.rs` - Widget management, event handling
- `libs/router/src/widget/nested.rs` - Nested route resolution
- `libs/router/src/url.rs` - URL parsing and caching

## Expected Performance Gains

- **Route parameter lookups**: 10-100x faster (O(n) → O(1))
- **Pattern matching**: 2-5x faster with early exits and caching
- **Event handling**: 5-10x faster by processing only active route
- **Memory usage**: 30-50% reduction with lazy widget loading
- **Navigation operations**: 2-3x faster with optimized history operations