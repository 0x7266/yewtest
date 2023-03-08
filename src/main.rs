use yew::prelude::*;

struct Model {
    count: i32,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model { count: 0 });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                count: state.count + 1,
            })
        })
    };

    html! {
        <div>
            <button {onclick}>{"+1"}</button>
            <p>{state.count}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
