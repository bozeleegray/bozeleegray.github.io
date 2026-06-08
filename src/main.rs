use yew::prelude::*;

/// A single area of work surfaced on the landing page.
struct Highlight {
    icon: &'static str,
    title: &'static str,
    blurb: &'static str,
}

const HIGHLIGHTS: &[Highlight] = &[
    Highlight {
        icon: "❄️",
        title: "Polar Cave Research",
        blurb: "Documenting glacier and carbonate cave systems across the high Arctic, \
                with an active scientific expedition program in Greenland and Svalbard.",
    },
    Highlight {
        icon: "🧭",
        title: "Field Expeditions",
        blurb: "Permitted, science-driven fieldwork — from logistics and partnerships \
                to on-ice survey and sample collection in remote high-latitude terrain.",
    },
    Highlight {
        icon: "🗺️",
        title: "High-Latitude GIS",
        blurb: "Arctic geology mapping, ship-path analysis, and cartographic design \
                in QGIS for planning and communicating polar work.",
    },
];

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div class="aurora" aria-hidden="true"></div>
            <main class="page">
                <header class="hero">
                    <p class="eyebrow">{ "Polar Exploration · Arctic Science" }</p>
                    <h1 class="title">{ "Boze Lee Gray" }</h1>
                    <p class="lede">
                        { "Field scientist mapping the frozen edges of the world — \
                           polar caves, Greenland expeditions, and high-latitude GIS." }
                    </p>
                    <div class="cta-row">
                        <a class="btn btn-primary" href="#work">{ "Explore the work" }</a>
                        <a class="btn btn-ghost" href="mailto:boze.l@northeastern.edu">
                            { "Get in touch" }
                        </a>
                    </div>
                </header>

                <section id="work" class="grid" aria-label="Areas of work">
                    { for HIGHLIGHTS.iter().map(render_highlight) }
                </section>

                <footer class="footer">
                    <span>{ "Built in Rust + WebAssembly" }</span>
                    <span class="dot" aria-hidden="true">{ "•" }</span>
                    <span>{ "© 2026 Boze Lee Gray" }</span>
                </footer>
            </main>
        </>
    }
}

fn render_highlight(h: &Highlight) -> Html {
    html! {
        <article class="card">
            <span class="card-icon" aria-hidden="true">{ h.icon }</span>
            <h2 class="card-title">{ h.title }</h2>
            <p class="card-blurb">{ h.blurb }</p>
        </article>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
