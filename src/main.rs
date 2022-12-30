use yew::prelude::*;
mod render;

use render::*;

#[function_component]
fn App() -> Html {
    let props = SimCanvasInfo {
        dim: (10.0, 10.0),
        bird_colour: "blue".to_string(),
        food_colour: "black".to_string(),
    };

    html! {
        <>
            <div>
                <div> <SimElement ..props.clone() /> </div>
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::Renderer::<App>::new().render();
}
