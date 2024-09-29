use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

#[function_component(CharacterSheet)]
pub fn character_sheet() -> Html {
    let canvas_ref = use_node_ref();

    use_effect_with_deps(|canvas_ref| {
        if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            draw_character_sheet(&context);
        }
        || ()
    }, canvas_ref.clone());

    html! {
        <canvas ref={canvas_ref} width="1000" height="1200" />
    }
}

fn draw_character_sheet(ctx: &CanvasRenderingContext2d) {
    // Set up basic styles
    ctx.set_font("20px Arial");
    ctx.set_fill_style(&JsValue::from_str("black"));

    // Draw character name
    ctx.fill_text("Name: Jujimufu", 10.0, 30.0).unwrap();

    // Draw level and class
    ctx.fill_text("Level: 11 | Class: Acrobolix", 10.0, 60.0).unwrap();

    // Draw experience
    ctx.fill_text("Experience: 64,969", 10.0, 90.0).unwrap();

    // Draw stats
    let stats = [
        ("STR", 13), ("DEX", 11), ("CON", 9),
        ("INT", 10), ("WIS", 11), ("CHA", 5)
    ];
    for (i, (stat, value)) in stats.iter().enumerate() {
        ctx.fill_text(&format!("{}: {}", stat, value), 10.0, 150.0 + (i as f64 * 30.0)).unwrap();
    }

    // Note: You'll need to add more drawing code to fully replicate the character sheet
}