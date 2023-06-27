use gloo::{net::http::Request, timers::callback::Timeout};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use stylist::{yew::styled_component, style};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::{button::Button, sale_amount_input::SaleInput, merchant_password_input::PasswordInput};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props
{
	pub merchant_uid: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct Sale
{
	phone: String,
	merchant_uid: String,
	purchase_price: Option<Decimal>,
	status: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MerchantAuthResponse
{
	success: bool,
	token: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PollResponse
{
	complete: bool,
	pending_sales: Option<Vec<Sale>>,
}

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Debug)]
#[store(storage = "local", storage_tab_sync)]
pub struct MerchantPersistentState
{
	pub token: Option<String>,
}

#[styled_component]
pub fn MerchantPortal(props: &Props) -> Html
{
	let token = use_store::<MerchantPersistentState>();
	let stylesheet = style!(r#"

		font-family: 'Bai Jamjuree', sans-serif;
		text-align: center;
		display: flex;
		flex-direction: column;
		height: 100vh;

		box-sizing: border-box;
		padding-left: 48px;
		padding-right: 48px;
		max-width: 100%;

		@media (max-width: 1024px) {
			font-size: 30px;
		}

		@media (max-width: 480px) {
		}

		@media (max-height: 1080px) {
			height: 50vh;
			font-size: 20px;
		}

		"#).unwrap();

	let img_style = style!(r#"
		img {
			margin-top: 100px;
			width: 20%;
			height: auto;
			@media (max-height: 1080px) {
				width: 5%;
			}
		}
		"#).unwrap();

	let price_input = use_state(|| Decimal::new(0, 0));
	let sale_alert_state = use_state(|| false);
	let sale_alert_state_clone = sale_alert_state.clone();
	let pending_sales_state = use_state(|| Vec::<Sale>::new());
	let pending_sales_state_clone = pending_sales_state.clone();
	let props_clone = props.clone();

	let token_clone = token.clone();
	let tc = token.clone();
	if !*sale_alert_state && token.0.token.is_some()
	{
		Timeout::new(1_000, move ||
		{
			let sale_alert_state_clone = sale_alert_state.clone();
			let pending_sales_state_clone = pending_sales_state.clone();
			let props = props_clone.clone();
			let token_clone = tc.clone();
			wasm_bindgen_futures::spawn_local(async move
			{
				let sale_alert_state = sale_alert_state_clone.clone();
				let pending_sales_state = pending_sales_state_clone.clone();

				let response = Request::post("https://api.rainyday.deals/merchant")
					.json(&serde_json::json!(
							{
								"request_type": "poll",
								"merchant_uid": props.merchant_uid,
								"merchant_token": token_clone.0.token.as_ref().unwrap()
							}
					))
					.unwrap()
					.send()
					.await;

				match response
				{
					Ok(res) =>
					{
						let poll_response = res.json::<PollResponse>().await.unwrap();
						if !poll_response.complete
						{
							pending_sales_state.set(poll_response.pending_sales.unwrap());
							sale_alert_state.set(true);
						} else
						{
							pending_sales_state.set(Vec::<Sale>::new());
							sale_alert_state.set(false);
						}
					},
					Err(_) =>
					{
					}
				}
			});
		}).forget();
	}

	let psales_for_callback = pending_sales_state_clone.clone();
	let props_clone = props.clone();
	let sale_alert_state_cloned = sale_alert_state_clone.clone();

	let piclone	= price_input.clone();

	let pass_state = use_state(|| String::new());
	let pass_clone = pass_state.clone();
	let on_password_input_change = Callback::from(move |input: String|
		{
			pass_state.set(input);
		});

	let merchant_uid = props.merchant_uid.clone();
	let token_clone = token_clone.clone();
	let on_login_submit = Callback::from(move |event: SubmitEvent|
		{
			event.prevent_default();
			let merchant_uid_clone = merchant_uid.clone();
			let pstate = pass_clone.clone();
			let token = token_clone.clone();
			wasm_bindgen_futures::spawn_local(async move
			{
				let response = Request::post("https://api.rainyday.deals/merchant")
					.json(&serde_json::json!(
							{
								"request_type": "get_auth",
								"merchant_uid": merchant_uid_clone,
								"password": *pstate
							}
					))
					.unwrap()
					.send()
					.await
					.unwrap()
					.json::<MerchantAuthResponse>()
					.await
					.unwrap();
				token.1.set(MerchantPersistentState { token: response.token });
			});
		});
	let on_sale_amount_change = Callback::from(move |input: String|
		{
			let input: String = input[1..].to_string();
			let mut price_decimal = Decimal::from_str_exact(&input).unwrap();
			price_decimal.set_scale(2).unwrap();
			price_input.set(price_decimal);
		});

	let token_clone = token.clone();
	let on_complete_sale = Callback::from(move |event: SubmitEvent|
		{
			event.prevent_default();

			let mut pending_sales = (*psales_for_callback).clone();

			let props = props_clone.clone();
			let sale_alert_state = sale_alert_state_cloned.clone();
			let pending_sales_state = psales_for_callback.clone();
			let piclone = piclone.clone();
			let token = token_clone.clone();
			wasm_bindgen_futures::spawn_local(async move
			{
				let price_input = piclone.clone();

				let _response = Request::post("https://api.rainyday.deals/merchant")
					.json(&serde_json::json!(
							{
								"request_type": "complete_sale",
								"merchant_uid": props.merchant_uid,
								"merchant_token": token.0.token.as_ref().unwrap().to_string(),
								"user_phone": pending_sales[0].phone,
								"price_of_sale": (*price_input).clone()
							}
					))
					.unwrap()
					.send()
					.await;

				pending_sales.remove(0);

				if pending_sales.is_empty()
				{
					sale_alert_state.set(false);
				}

				pending_sales_state.set(pending_sales);
			});
		});

	html! {
		<div class={stylesheet}>
			<h1>{ "Merchant Portal" }</h1>
			if !token.0.token.is_none()
			{
				<h1>{ "Please Log In" }</h1>
				<form onsubmit={on_login_submit}>
					<PasswordInput onchange={on_password_input_change}/>
					<Button title={"Log In"} />
				</form>
			}
			else
			{
				if *sale_alert_state_clone
				{
					<h1>{ "You'll be notified here when a customer scans your QR Code" }</h1>
					<p>{ format!("Merchant UID: {}", props.merchant_uid) }</p>
				}
				else
				{
					<h1>{ format!("Pending Sale") }</h1>
					//<h2>{ format!("{}", (&(*pending_sales_state_clone)[0].phone)) }</h2>
					<h2>{ format!("(646)-591-9552") }</h2>
					<h2>{ format!("Input Full Sale Price") }</h2>
					<div>
						<form onsubmit={on_complete_sale}>
							<SaleInput onchange={on_sale_amount_change}/>
							<Button title={"Complete Sale"} />
						</form>
					</div>
				}
			}
			<div class={img_style}>
				<img src="img/logo.png" alt="logo"/> 
			</div>
		</div>
	}
}

