use gloo::{console::log, net::http::Request};
use serde_json::json;
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
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move
                {
                    let response = Request::post("https://api.rainyday.deals/sms-login")
                        .json(&json!(
                            {
                                "phone": *state,
                                "asking_for_code": true
                            }
                        ))
                        .unwrap()
                        .send()
                        .await
                        .unwrap();
                    log!(format!("response: {:?}", response));
                })
            });

    html!
    {
        <div class = {stylesheet}>
            <PhoneInput onchange={onchange}/>
            <Button onsubmit={onsubmit}/>
        </div>
    }
}
