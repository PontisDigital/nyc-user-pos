use gloo::net::http::Request;
use serde_json::json;
use yew::prelude::*;
use yewdux::prelude::*;
use stylist::{yew::styled_component, style, Style};

use crate::{pages::user_pos::{PersistentState, Merchant}, components::button::Button};

#[derive(serde::Serialize, serde::Deserialize)]
struct RequestSent
{
	success: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
	pub	merchant: Merchant,
}

#[styled_component]
pub fn LoggedInPOS(props: &Props) -> Html
{
	let auth = use_store::<PersistentState>().0;

	let submitted_state = use_state(|| false);

	let stylesheet: Style = style!(r#"

		box-sizing: border-box;
		padding-left: 48px;
		padding-right: 48px;
		max-width: 100%;

	"#).unwrap();

	let phone = auth.phone.clone().unwrap();
	let token = auth.token.clone().unwrap();
	let merchant_uid = props.merchant.uid.clone();

	let submitted_state_clone = submitted_state.clone();
	let onsubmit = Callback::from(move |event: SubmitEvent|
	{
		event.prevent_default();
		let phone = phone.clone();
		let token = token.clone();
		let merchant_uid = merchant_uid.clone();
		let submitted_state = submitted_state_clone.clone();
		wasm_bindgen_futures::spawn_local(async move
		{
			let result = Request::post("https://api.rainyday.deals/sms-login")
				.json(&json!(
				{
					"purchase_req": {
					  "phone": phone,
					  "token": token,
					  "merchant_uid": merchant_uid
					}
				}
				))
				.unwrap()
				.send()
				.await
				.unwrap();
			let success = result.json::<RequestSent>().await.unwrap().success;
			submitted_state.set(success);
		});
	});

	html!
	{
		<div class = {stylesheet}>
			if !*submitted_state
			{
				<h1>
				{
					format!("Making a purchase at {}?", props.merchant.name)
				}
				</h1>

				<form onsubmit={onsubmit}>
					<Button title={"Confirm"}/>
				</form>
			}
			else
			{
				<h1>
				{
					format!("Purchase request sent to {}!", props.merchant.name)
				}
				</h1>
			}
		</div>
	}
}
