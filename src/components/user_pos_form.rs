use gloo::net::http::Request;
use serde_json::json;
use web_sys::window;
use yew::prelude::*;
use yewdux::prelude::*;
use crate::{components::{phone_number_input::PhoneInput, button::Button, verification_code_input::CodeInput}, pages::user_pos::AnonUserPersistentState};
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
	let dispatch = use_store::<AnonUserPersistentState>().1;

	let stylesheet: Style = style!(r#"

		box-sizing: border-box;
		padding-left: 48px;
		padding-right: 48px;
		max-width: 100%;

	"#).unwrap();

	let button_enabled = use_state(|| true);
	let be = button_enabled.clone();
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
				pistate.set(input);
			}
			else
			{
				cistate.set(input);
			}
		});

	let pistate = phone_input_state.clone();
	let cistate = code_input_state.clone();
	let sent_state = code_sent_state.clone();

	let verify_response = use_state(|| CodeVerifiedResponse { verified: false, token: "".to_string() });
	let verify_response_clone = verify_response.clone();

	let dispatch = dispatch.clone();
	let onsubmit = Callback::from(move |event: SubmitEvent|
		{
			be.set(false);
			let be = be.clone();
			let dispatch = dispatch.clone();
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
								"auth": {
									"phone": *pistate,
								}
							}
						))
						.unwrap()
						.send()
						.await
						.unwrap();
					let sent = response.json::<CodeSubmitResponse>().await.unwrap().sent;
					sent_state.set(sent);
					be.set(true);
				})
			}
			else // dealing with verification code field
			{
				let verify_response_clone = verify_response_clone.clone();
				wasm_bindgen_futures::spawn_local(async move
				{
					let response = Request::post("https://api.rainyday.deals/sms-login")
						.json(&json!(
							{
								"auth": {
									"phone": *pistate,
									"code": *cistate,
								}
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
								dispatch.set(AnonUserPersistentState {
									token: Some(vfied.token),
									uid: Some((*pistate).clone()),
								});
							}
						},
						Err(_) =>
						{
						}
					}
				})
			}
		});

	let center = style!(r#"
		display: flex;
		justify-content: center;
		"#).unwrap();

	let tohomepage = Callback::from(move |event: MouseEvent|
		{
			event.prevent_default();
			window().unwrap().location().set_href("/").unwrap();
		});

	html!
	{
		<div class = {stylesheet}>
		if !*code_sent_state
		{
			<form onsubmit={onsubmit}>
				<PhoneInput onchange={onchange}/>
				<Button title={"Submit"} disabled={!*button_enabled}/>
			</form>
		}
		else
		{
			if !verify_response.verified
			{
				<h1>{"Enter the code we sent you"}</h1>
				<form onsubmit={onsubmit} autocomplete={"off"}>
					<CodeInput onchange={onchange}/>
					<Button title={"Submit"} disabled={!*button_enabled}/>
				</form>
			}
			else
			{
				<h1>{"Success!"}</h1>

				<script src={"https://unpkg.com/@lottiefiles/lottie-player@latest/dist/lottie-player.js"}></script>
				<div class={center}>
					<lottie-player src={"https://assets3.lottiefiles.com/packages/lf20_SFdTxf9D07.json"}  background={"transparent"}  speed={"0.5"}  style={"width: 300px; height: 300px;"}  loop=false controls=false autoplay=true></lottie-player>
				</div>

				<form onsubmit={onsubmit}>
					<Button title={"Return to Homepage"} on_click={tohomepage}/>
				</form>
			}
		}
		</div>
	}
}
