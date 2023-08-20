use cgt::{
    numeric::rational::Rational,
    short::partizan::{
        games::snort::Snort, partizan_game::PartizanGame, transposition_table::TranspositionTable,
    },
};
use std::fmt::Write;
use sycamore::{prelude::*, web::html};
use viz_js::VizInstance;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Clone, Copy)]
pub struct SnortState<'a> {
    position_history: &'a Signal<Vec<Snort>>,
    cache: &'a ReadSignal<TranspositionTable<Snort>>,
}

impl<'a> SnortState<'a> {
    pub fn new(cx: Scope<'a>, position: Snort) -> Self {
        let position_history = create_signal(cx, vec![position]);
        let cache = create_signal(cx, TranspositionTable::new());
        Self {
            position_history,
            cache,
        }
    }
}

#[component(inline_props)]
pub async fn Snort<'a, G: Html>(cx: Scope<'a>, state: SnortState<'a>) -> View<G> {
    let left_moves = state
        .position_history
        .map(cx, |pos| pos.last().unwrap_throw().left_moves());
    let right_moves = state
        .position_history
        .map(cx, |pos| pos.last().unwrap_throw().right_moves());
    // .map(cx, |pos| pos.last().unwrap_throw().sensible_right_moves(&state.cache.get()));

    let set_pos = move |new| {
        log::info!("Selected {:?}", new);
        let mut old = state.position_history.modify();
        old.push(new);
    };

    let history = View::new_dyn(cx, move || {
        View::new_fragment(
            map_indexed(cx, state.position_history, move |cx, pos| {
                let canonical_form = pos.canonical_form(&state.cache.get());
                let temperature = canonical_form.temperature();
                let degree = pos.graph.degree();
                let fitness = temperature - Rational::from(degree as i32);
                let edges = {
                    let mut first = true;
                    let mut buf = String::new();

                    for v in pos.graph.vertices() {
                        for u in pos.graph.vertices() {
                            if v < u && pos.graph.are_adjacent(v, u) {
                                if !first {
                                    write!(buf, ",").unwrap();
                                }
                                write!(buf, "({},{})", v, u).unwrap();
                                first = false;
                            }
                        }
                    }

                    buf
                };

                let dot: &Signal<String> = create_signal(cx, pos.to_graphviz());
                let dot: &ReadSignal<String> = &*dot;

                html::div()
                    .class("flex gap-x-1")
                    .dyn_c(move || View::new_dyn(cx, move || view! {cx, SnortPosition(dot=dot)}))
                    .c(html::div()
                        .class("flex flex-col gap-y-1")
                        .c(html::span()
                            .class("text-white font-mono")
                            .dyn_t(move || format!("Canonical Form: {}", canonical_form)))
                        .c(html::span()
                            .class("text-white font-mono")
                            .dyn_t(move || format!("Temperature: {}", temperature))))
                    .view(cx)
            })
            .get()
            .as_ref()
            .clone(),
        )
    });

    html::div()
        .c(history)
        .c(html::div().class("").c(html::div()
            .class("gap-2")
            .c(html::span().t("Left:").class("text-white font-mono"))
            .c(html::div()
                .class("flex gap-x-1")
                .c(View::new_dyn(cx, move || {
                    View::new_fragment(
                        map_indexed(cx, left_moves, move |cx, pos| {
                            let dot: &Signal<String> = create_signal(cx, pos.to_graphviz());
                            let dot: &ReadSignal<String> = &*dot;
                            let pos2 = pos.clone();

                            html::div()
                                .c(view! {cx, SnortPosition(dot=dot)})
                                .c(html::span()
                                    .dyn_t(move || {
                                        format!(
                                            "{}",
                                            pos2.clone().canonical_form(&state.cache.get())
                                        )
                                    })
                                    .class("text-white font-mono"))
                                .on("click", move |_| set_pos(pos.clone()))
                                .view(cx)
                        })
                        .get()
                        .as_ref()
                        .clone(),
                    )
                })))
            .c(html::span().t("Right:").class("text-white font-mono"))
            .c(html::div()
                .class("flex gap-x-1")
                .c(View::new_dyn(cx, move || {
                    View::new_fragment(
                        map_indexed(cx, right_moves, move |cx, pos| {
                            let dot: &Signal<String> = create_signal(cx, pos.to_graphviz());
                            let dot: &ReadSignal<String> = &*dot;
                            let pos2 = pos.clone();

                            html::div()
                                .c(view! {cx, SnortPosition(dot=dot)})
                                .c(html::span()
                                    .dyn_t(move || {
                                        format!(
                                            "{}",
                                            pos2.clone().canonical_form(&state.cache.get())
                                        )
                                    })
                                    .class("text-white font-mono"))
                                .on("click", move |_| set_pos(pos.clone()))
                                .view(cx)
                        })
                        .get()
                        .as_ref()
                        .clone(),
                    )
                })))))
        .view(cx)
}

#[component(inline_props)]
pub async fn SnortPosition<'a, G: Html>(cx: Scope<'a>, dot: &'a ReadSignal<String>) -> View<G> {
    let graphviz = VizInstance::new().await;
    let svg = graphviz
        .render_svg_element((dot.get().as_ref()).to_owned(), viz_js::Options::default())
        .expect_throw("Could not render graphviz");

    html::svg()
        .attr("height", "300")
        .attr("viewBox", svg.get_attribute("viewBox").unwrap())
        .dyn_dangerously_set_inner_html(move || svg.inner_html())
        .view(cx)
}
