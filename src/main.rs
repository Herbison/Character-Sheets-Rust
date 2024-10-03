// main.rs and lib.rs form a single crate.
// main.rs is the entry point for the binary, while lib.rs defines the library.

// This imports all items from the yew prelude, giving us access to Yew's essential features.
use yew::prelude::*;

// We don't need to specify 'use crate::CharacterSheet;' because items in lib.rs are automatically available throughout the project.
use character_sheet::CharacterSheet;

// This defines our main App component
#[function_component(App)]
fn app() -> Html {
    html! {
        <CharacterSheet />
    }
}

fn main() {
    // This line starts the Yew application by rendering our App component
    yew::Renderer::<App>::new().render();
}