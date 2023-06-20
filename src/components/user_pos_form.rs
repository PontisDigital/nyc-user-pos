use gloo::{console::log, net::http::Request};
use serde_json::json;
use yew::prelude::*;
use crate::components::{phone_number_input::PhoneInput, button::Button, verification_code_input::CodeInput};
use stylist::{yew::styled_component, style, Style};

#[derive(serde::Serialize, serde::Deserialize)]
struct CodeSubmitResponse
{
    sent: bool,
}

#[styled_component]
pub fn UserPOSForm() -> Html
{
    let stylesheet: Style = style!(r#"

        box-sizing: border-box;
        padding: 48px;

    "#).unwrap();

    let input_state = use_state(|| "".to_string());
    let code_sent_state = use_state(|| false);

    let state = input_state.clone();
    let onchange = Callback::from(move |input|
        {
            log!(format!("phone changed to: {}", input));
            state.set(input);
        });

    let state = input_state.clone();
    let sent_state = code_sent_state.clone(); let onsubmit = Callback::from(move |event: SubmitEvent|
        {
            event.prevent_default();
            log!(format!("submitted {}", *state));
            let state = state.clone();
            let sent_state = sent_state.clone();
            wasm_bindgen_futures::spawn_local(async move
                {
                    let response = Request::post("https://api.rainyday.deals/sms-login")
                        .json(&json!(
                            {
                                "phone": *state,
                                "asking_for_code": true,
                                "testing": true
                            }
                        ))
                        .unwrap()
                        .send()
                        .await
                        .unwrap();
                    let sent = response.json::<CodeSubmitResponse>().await.unwrap().sent;
                    sent_state.set(sent);
                })
            });

    html!
    {
        <div class = {stylesheet}>
        if *code_sent_state
        {
            <form onsubmit={onsubmit}>
                <PhoneInput onchange={onchange}/>
                <Button />
            </form>
        }
        else
        {
            <h1>{"Enter the code we sent you"}</h1>
            <CodeInput onchange={onchange}/>
        }
        </div>
    }
}
