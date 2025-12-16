## Router Roadmap

### Phase 1: API Completeness + Semantics
- [x] Add `replace`, `forward`, `can_go_forward`, `reset`, `clear_history`, `depth` to `RouterWidget` / `RouterWidgetRef`.
- [x] Add stack semantics: `push(route_id)`, `pop()`, `pop_to(route_id)`, `pop_to_root()`, `set_stack(Vec<Route>)`.
- [x] Emit `RouterAction` into `Actions` on route changes (no callbacks required).

### Phase 2: Nested Routing (Real)
- [ ] Delegate remainder paths to child routers (e.g. `/admin/*` passes the tail into `admin_router`) and support optional “base path” composition.
- [ ] Add route “not found” handling at each router level (configurable fallback route id).

### Phase 3: Transitions / Animations
- [ ] Implement animated transitions inside `RouterWidget` (keep old+new alive during transition, then drop old).
- [ ] Built-in presets: `None`, `Fade`, `SlideLeft/Right`, `Scale`, `SharedAxis` with push/pop direction.
- [ ] Per-route and per-navigation overrides (e.g. push slides, replace fades).

### Phase 4: URL + Deep Linking (Web + Desktop optional)
- [ ] Parse + generate paths (including query + hash), synchronize with web history, and allow initial route from URL.
- [ ] Add `navigate_by_url(url)` and `current_url()` helpers.

### Phase 5: Guards, Redirects, Middleware
- [ ] Route guards (sync + async) for auth/feature flags; redirect/replace behaviors.
- [ ] “Before leave” confirmation hooks.

### Phase 6: State + Data
- [ ] Typed params/query support (keep current `LiveId` path, add string map for query).
- [ ] Persistence of history stack + current route (extend to params/query).

### Phase 7: Tooling + Testing
- [ ] Add unit tests for pattern parsing/matching, precedence/collisions, nested delegation.
- [ ] Add a small “router inspector” debug overlay for current route/stack/params.
