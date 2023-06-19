use gloo::console::log;
use stylist::{yew::styled_component, style};
use yew::prelude::*;
use crate::components::{phone_number_input::PhoneInput, button::Button};

#[styled_component]
pub fn UserPOSForm() -> Html
{
    let stylesheet = style!(r#"

        box-sizing: border-box;
        padding: 48px;

    "#).unwrap();

    let input_state = use_state(|| "".to_string());

    let state = input_state.clone();
    let onchange = Callback::from(move |input|
        {
            log!(format!("phone changed to: {}", input));
            state.set(input);
        });

    let state = input_state.clone();
    let onsubmit = Callback::from(move |event: MouseEvent|
        {
            event.prevent_default();
            log!(format!("submitted {}", *state));
        });

    html!
    {
        <div class = {stylesheet}>
            <PhoneInput onchange={onchange}/>
            <Button onsubmit={onsubmit}/>
        </div>
    }
}
