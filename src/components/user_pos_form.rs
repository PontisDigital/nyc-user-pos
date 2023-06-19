use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::{phone_number_input::PhoneInput, button::Button};

#[styled_component]
pub fn UserPOSForm() -> Html
{
    html!
    {
        <div>
            <PhoneInput />
            <Button />
        </div>
    }
}
