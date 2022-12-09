use yew::prelude::*;
mod render;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    // let canvas = yew::prelude::AnimationEvent::new(type_)

    let x = html! {
        <>
            <h1> { "TITLE" } </h1>
            <div>
                <button {onclick}>{ "+1" }</button>
                <p>{ *counter }</p>
            </div>
        </>
    };

    x
}

fn main() {
    yew::Renderer::<App>::new().render();
}