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
    html!
    {
        <div class = {stylesheet}>
            <PhoneInput />
            <Button />
        </div>
    }
}
