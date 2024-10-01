use yew::prelude::*;

#[function_component(CharacterSheet)]
pub fn character_sheet() -> Html {
    html! {
        <div class="character-sheet">
            <div class="name-box">
                <h1>{"BarBar"}</h1>
            </div>
            <img src="images//Barbar1.webp" alt="BarBar Smash" class="BarBar"/>
            <div class="info">
                <p>{"Level: 2 | Class: Barbellian"}</p>
            </div>
            <div class="stats">
                <ul>
                    <li>{"Health: 9"}</li>
                    <li>{"Stamina: 4"}</li>
                    <li>{"Strength: 12"}</li>
                    <li>{"Mobility: 7"}</li>
                    <li>{"Intelligence: 2"}</li>
                    <li>{"Wisdom: 1"}</li>
                    <li>{"Focus: 10"}</li>
                </ul>
            </div>
        </div>
    }
}

// Not going with Canvas

// use wasm_bindgen::prelude::*;
// use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

// #[function_component(CharacterSheet)]
// pub fn character_sheet() -> Html {
//     let canvas_ref = use_node_ref();

//     use_effect_with_deps(|canvas_ref| {
//         if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
//             let context = canvas
//                 .get_context("2d")
//                 .unwrap()
//                 .unwrap()
//                 .dyn_into::<CanvasRenderingContext2d>()
//                 .unwrap();

//             draw_character_sheet(&context);
//         }
//         || ()
//     }, canvas_ref.clone());

//     html! {
//         <canvas ref={canvas_ref} width="1280" height="1502" />
//     }
// }

// fn draw_character_sheet(ctx: &CanvasRenderingContext2d) {
//     // Set up basic styles
//     ctx.set_font("20px Arial");
//     ctx.set_fill_style(&JsValue::from_str("black"));

//     // Draw character name
//     ctx.fill_text("BarBar", 50.0, 30.0).unwrap();

//     // Draw level and class
//     ctx.fill_text("Level: 2 | Class: Barbellian", 10.0, 60.0).unwrap();

//     // Draw stats
//     let stats: [(&str, i32); 7] = [
//         ("Health", 9), ("Stamina", 4), ("Strength", 12),
//         ("Mobility", 7), ("Intelligence", 2), ("Wisdom", 1),
//         ("Focus", 10)
//     ];
//     for (i, (stat, value)) in stats.iter().enumerate() {
//         ctx.fill_text(&format!("{}: {}", stat, value), 10.0, 150.0 + (i as f64 * 30.0)).unwrap();
//     }
// }