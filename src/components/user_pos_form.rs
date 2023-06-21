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

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct CodeVerifiedResponse
{
	#[serde(rename = "success")]
	verified: bool,
	token: String,
}

#[styled_component]
pub fn UserPOSForm() -> Html
{
	let stylesheet: Style = style!(r#"

		box-sizing: border-box;
		padding-left: 48px;
		padding-right: 48px;
		max-width: 100%;

	"#).unwrap();

	let phone_input_state = use_state(|| "".to_string());
	let code_input_state = use_state(|| "".to_string());
	let code_sent_state = use_state(|| false);
	let cstate = code_sent_state.clone();
	let cistate = code_input_state.clone();
	let pistate = phone_input_state.clone();
	let onchange = Callback::from(move |input|
		{
			if !*cstate
			{
				log!(format!("phone changed to: {}", input));
				pistate.set(input);
			}
			else
			{
				log!(format!("code changed to: {}", input));
				cistate.set(input);
			}
		});

	let pistate = phone_input_state.clone();
	let cistate = code_input_state.clone();
	let sent_state = code_sent_state.clone();

	let verify_response = use_state(|| CodeVerifiedResponse { verified: false, token: "".to_string() });
	let verify_response_clone = verify_response.clone();

	let onsubmit = Callback::from(move |event: SubmitEvent|
		{
			event.prevent_default();
			let pistate = pistate.clone();
			let cistate = cistate.clone();
			let sent_state = sent_state.clone();

			if !*sent_state //dealing with phone input field
			{
				wasm_bindgen_futures::spawn_local(async move
				{
					let response = Request::post("https://api.rainyday.deals/sms-login")
						.json(&json!(
							{
								"phone": *pistate,
								"testing": true,
							}
						))
						.unwrap()
						.send()
						.await
						.unwrap();
					let sent = response.json::<CodeSubmitResponse>().await.unwrap().sent;
					sent_state.set(sent);
				})
			}
			else // dealing with verification code field
			{
				let cistate = cistate.clone();
				let verify_response_clone = verify_response_clone.clone();
				wasm_bindgen_futures::spawn_local(async move
				{
					let response = Request::post("https://api.rainyday.deals/sms-login")
						.json(&json!(
							{
								"phone": *pistate,
								"code": *cistate,
								"testing": true,
							}
						))
						.unwrap()
						.send()
						.await
						.unwrap();
					let verified = response.json::<CodeVerifiedResponse>().await;
					match verified
					{
						Ok(vfied) =>
						{
							verify_response_clone.set(vfied.clone());
							if vfied.verified
							{
								log!("verified");
								log!(format!("token: {:?}", vfied));
							}
						},
						Err(_) =>
						{
						}
					}
				})
			}
		});

	html!
	{
		<div class = {stylesheet}>
		if !*code_sent_state
		{
			<form onsubmit={onsubmit}>
				<PhoneInput onchange={onchange}/>
				<Button />
			</form>
		}
		else
		{
			if !verify_response.verified
			{
				<h1>{"Enter the code we sent you"}</h1>
				<form onsubmit={onsubmit}>
					<CodeInput onchange={onchange}/>
					<Button />
				</form>
			}
			else
			{
				<h1>{"Code verified!"}</h1>
				<h2>{&verify_response.token}</h2>
			}
		}
		</div>
	}
}
