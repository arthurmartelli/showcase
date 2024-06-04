use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[styled_component]
pub fn App() -> Html {
    let style = style! {
        text-align: center;
    }
    .expect("Unable to parse css");

    html! {
        <>
            <h1 class={style}>{"Hello!"}</h1>
            <form>
                <h2>{"Create new post"}</h2>
                <input type="text" />
            </form>
        </>
    }
}
