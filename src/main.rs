use yew::prelude::*;
use character_sheet::CharacterSheet;

#[function_component(App)]
fn app() -> Html {
    html! {
        <CharacterSheet />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}