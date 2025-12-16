use crate::{route::Route, router::Router};
use makepad_widgets::*;

live_design! {
    pub RouterWidgetBase = {{RouterWidget}} {}
    pub RouterWidget = <RouterWidgetBase> {
        width: Fill, height: Fill
    }
}

/// Router widget for managing navigation between pages
#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct RouterWidget {
    #[rust]
    area: Area,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[live]
    active_route: LiveId,
    #[live]
    default_route: LiveId,
    #[live(false)]
    persist_state: bool,
    #[rust]
    router: Router,
    #[rust]
    route_templates: ComponentMap<LiveId, LivePtr>,
    #[rust]
    route_widgets: ComponentMap<LiveId, WidgetRef>,
}

impl LiveHook for RouterWidget {
    fn before_apply(
        &mut self,
        _cx: &mut Cx,
        apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        if let ApplyFrom::UpdateFromDoc { .. } = apply.from {
            self.route_templates.clear();
        }
    }

    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        match apply.from {
            ApplyFrom::NewFromDoc { .. } | ApplyFrom::UpdateFromDoc { .. } => {
                if self.router.current_route().is_none() {
                    let initial_route = if self.active_route.0 != 0 {
                        self.active_route
                    } else {
                        self.default_route
                    };

                    if initial_route.0 != 0 {
                        self.router = if self.persist_state {
                            Router::with_persistence(Route::new(initial_route))
                        } else {
                            Router::new(Route::new(initial_route))
                        };
                        self.active_route = initial_route;
                    }
                }

                if let Some(ptr) = self.route_templates.get(&self.active_route) {
                    self.route_widgets
                        .get_or_insert(cx, self.active_route, |cx| {
                            WidgetRef::new_from_ptr(cx, Some(*ptr))
                        });
                }
            }
            _ => (),
        }
    }

    fn apply_value_instance(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        let id = nodes[index].id;
        match apply.from {
            ApplyFrom::NewFromDoc { file_id } | ApplyFrom::UpdateFromDoc { file_id, .. } => {
                if nodes[index].origin.has_prop_type(LivePropType::Instance) {
                    let live_ptr = cx
                        .live_registry
                        .borrow()
                        .file_id_index_to_live_ptr(file_id, index);
                    self.route_templates.insert(id, live_ptr);

                    if let Some(widget) = self.route_widgets.get_mut(&id) {
                        widget.apply(cx, apply, index, nodes);
                    }
                } else {
                    cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                }
            }
            _ => (),
        }
        nodes.skip_node(index)
    }
}

impl RouterWidget {
    pub fn navigate(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        if self.route_templates.contains_key(&route_id) {
            self.router.navigate_to(route_id);
            self.active_route = route_id;

            if let Some(ptr) = self.route_templates.get(&route_id) {
                self.route_widgets
                    .get_or_insert(cx, route_id, |cx| WidgetRef::new_from_ptr(cx, Some(*ptr)));
            }

            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    pub fn back(&mut self, cx: &mut Cx) -> bool {
        if self.router.back() {
            if let Some(route) = self.router.current_route() {
                self.active_route = route.id;

                if let Some(ptr) = self.route_templates.get(&route.id) {
                    self.route_widgets
                        .get_or_insert(cx, route.id, |cx| WidgetRef::new_from_ptr(cx, Some(*ptr)));
                }

                self.redraw(cx);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn can_go_back(&self) -> bool {
        self.router.can_go_back()
    }
}

impl WidgetNode for RouterWidget {
    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    fn area(&self) -> Area {
        self.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.area.redraw(cx)
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        for (route_id, widget) in self.route_widgets.iter() {
            if path.len() > 0 && path[0] == *route_id {
                if path.len() == 1 {
                    results.push(widget.clone());
                } else {
                    widget.find_widgets(&path[1..], cached, results);
                }
            } else {
                widget.find_widgets(path, cached, results);
            }
        }
    }

    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for widget in self.route_widgets.values() {
            let result = widget.uid_to_widget(uid);
            if !result.is_empty() {
                return result;
            }
        }
        WidgetRef::empty()
    }
}

impl Widget for RouterWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if let Some(widget) = self.route_widgets.get_mut(&self.active_route) {
            let widget_uid = widget.widget_uid();
            cx.group_widget_actions(uid, widget_uid, |cx| widget.handle_event(cx, event, scope));
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);

        if let Some(widget) = self.route_widgets.get_mut(&self.active_route) {
            widget.draw_all(cx, scope);
        }

        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

impl RouterWidgetRef {
    pub fn navigate(&self, cx: &mut Cx, route_id: LiveId) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.navigate(cx, route_id)
        } else {
            false
        }
    }

    pub fn back(&self, cx: &mut Cx) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.back(cx)
        } else {
            false
        }
    }

    pub fn can_go_back(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.can_go_back()
        } else {
            false
        }
    }
}
