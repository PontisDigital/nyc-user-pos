use gloo::net::http::Request;
use serde_json::json;
use yew::prelude::*;
use yewdux::prelude::*;
use stylist::{yew::styled_component, style, Style};

use crate::{pages::user_pos::{UserPersistentState, Merchant}, components::{button::Button, user_price_form::UserPriceForm}};

#[derive(serde::Serialize, serde::Deserialize)]
struct RequestSent
{
	success: bool,
	error: Option<String>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
	pub	merchant: Merchant,
}

#[styled_component]
pub fn LoggedInPOS(props: &Props) -> Html
{
	let auth = use_store::<UserPersistentState>().0;

	let submitted_state = use_state(|| false);
	let someone_in_front = use_state(|| false);
	let sif = someone_in_front.clone();

	let center = style!(r#"
		display: flex;
		justify-content: center;
		"#).unwrap();
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

	let dispatch = use_store::<UserPersistentState>().1;
	let onsubmit = Callback::from(move |event: SubmitEvent|
	{
		event.prevent_default();
		let phone = phone.clone();
		let token = token.clone();
		let merchant_uid = merchant_uid.clone();
		let submitted_state = submitted_state_clone.clone();
		let dclone = dispatch.clone();
		let someone_in_front = someone_in_front.clone();
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
			let response = result.json::<RequestSent>().await.unwrap();
			if response.error.is_some()
			{
				if response.error.unwrap() == "pending"
				{
					someone_in_front.set(true);
				}
				else
				{
					dclone.set(UserPersistentState
					{
						token: None,
						phone: None,
					});
				}
			}
			submitted_state.set(response.success);
		});
	});

	html!
	{
		<div class = {stylesheet}>
			if !*submitted_state
			{
				if props.merchant.use_phone
				{
					if *sif
					{
						<h1>
						{format!("Someone is in front of you at {}", props.merchant.name)}
						</h1>
						<h1>{"Please wait until you're at the front of the line to hit the button"}</h1>
					}
					else
					{
						<h1>
						{format!("Making a purchase at {}?", props.merchant.name)}
						</h1>
					}
				}

				if !props.merchant.use_phone
				{
					<UserPriceForm merchant={props.merchant.clone()}/>
				}
				else
				{
					<form onsubmit={onsubmit}>
						<Button title={"Save now"}/>
					</form>
				}
			}
			else
			{
				if props.merchant.use_phone
				{
					<h1>
						{format!("Purchase request sent to {}!", props.merchant.name)}
					</h1>
					<script src={"https://unpkg.com/@lottiefiles/lottie-player@latest/dist/lottie-player.js"}></script>
					<div class={center}>
						<lottie-player src={"https://assets3.lottiefiles.com/packages/lf20_SFdTxf9D07.json"}  background={"transparent"}  speed={"0.5"}  style={"width: 300px; height: 300px;"}  loop=false controls=false autoplay=true></lottie-player>
					</div>
				}
				else
				{
					<h1>
						{format!("Purchase at {} complete!", props.merchant.name)}
					</h1>
				}
			}
		</div>
	}
}
