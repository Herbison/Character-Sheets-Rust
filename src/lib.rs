use yew::prelude::*;

// CharacterSheet is a function component that renders the character's information
#[function_component(CharacterSheet)]
pub fn character_sheet() -> Html {
    // Define character stats
    // u8 is an 8-bit unsigned integer, suitable for storing character attributes from 0 to 255
    let health: u8 = 9;
    let stamina: u8 = 4;
    let strength: u8 = 12;
    let mobility: u8 = 7;
    let intelligence: u8 = 2;
    let wisdom: u8 = 1;
    let focus: u8 = 10;

    // The html! macro is used to create the component's HTML structure
    html! {
        // Main container for the character sheet
        <div class="character-sheet">
            // Character name image
            <img src="images//BarbarName.png" alt="BarBar Name" class="BarBar"/>
            // Character main image
            <img src="images//BarbarMain.webp" alt="BarBar Smash" class="BarBar"/>
            // Container for character info (level and class)
            <div class="info">
                <h3>{"Level: 2"}</h3>
                <h2>{"Class: Barbellian"}</h2>
            </div>
            // Container for character stats
            <div class="stats">
                // Unordered list to display stats
                <ul>
                    // List items for each stat
                    // format! macro is used to combine the stat name and its value. Like an fstring in python.
                    <li>{format!("Health: {}", health)}</li>
                    <li>{format!("Stamina: {}", stamina)}</li>
                    <li>{format!("Strength: {}", strength)}</li>
                    <li>{format!("Mobility: {}", mobility)}</li>
                    <li>{format!("Intelligence: {}", intelligence)}</li>
                    <li>{format!("Wisdom: {}", wisdom)}</li>
                    <li>{format!("Focus: {}", focus)}</li>
                </ul>
            </div>
        </div>
    }
}