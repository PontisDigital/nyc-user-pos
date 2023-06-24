use gloo::{net::http::Request, timers::callback::Timeout, console::log};
use rust_decimal::Decimal;
use stylist::{yew::styled_component, style};
use yew::prelude::*;

use crate::components::button::Button;

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
pub struct PollResponse
{
	complete: bool,
	pending_sales: Option<Vec<Sale>>,

}

#[styled_component]
pub fn MerchantPortal(props: &Props) -> Html
{
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
	
	let sale_alert_state = use_state(|| false);
	let sale_alert_state_clone = sale_alert_state.clone();
	let pending_sales_state = use_state(|| Vec::<Sale>::new());
	let pending_sales_state_clone = pending_sales_state.clone();
	let props_clone = props.clone();

	if !*sale_alert_state
	{
		Timeout::new(1_000, move ||
		{
			let sale_alert_state_clone = sale_alert_state.clone();
			let pending_sales_state_clone = pending_sales_state.clone();
			let props = props_clone.clone();
			wasm_bindgen_futures::spawn_local(async move
			{
				let sale_alert_state = sale_alert_state_clone.clone();
				let pending_sales_state = pending_sales_state_clone.clone();

				let response = Request::post("https://api.rainyday.deals/merchant")
					.json(&serde_json::json!(
							{
								"request_type": "poll",
								"merchant_uid": props.merchant_uid
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
						}
						else
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

	let on_complete_sale = Callback::from(move |event: MouseEvent|
		{
			event.prevent_default();

			let mut pending_sales = (*psales_for_callback).clone();

			let props = props_clone.clone();
			let sale_alert_state = sale_alert_state_cloned.clone();
			let pending_sales_state = psales_for_callback.clone();
			wasm_bindgen_futures::spawn_local(async move
			{

				log!("Completing sale for ", &pending_sales[0].phone);

				let response = Request::post("https://api.rainyday.deals/merchant")
					.json(&serde_json::json!(
							{
								"request_type": "complete_sale",
								"merchant_uid": props.merchant_uid,
								"merchant_token": "".to_string(),
								"user_phone": pending_sales[0].phone,
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
			if !*sale_alert_state_clone
			{
				<h1>{ "You'll be notified here when a customer scans your QR Code" }</h1>
				<p>{ format!("Merchant UID: {}", props.merchant_uid) }</p>
			}
			else
			{
				<h1>{ format!("SALE ALERT!!!") }</h1>
				<p>{ format!("Pending Sale: {:?}", (&(*pending_sales_state_clone)[0])) }</p>
				<div>
					<Button title={"Complete Sale"} on_click={on_complete_sale}/>
				</div>
			}
			<div class={img_style}>
				<img src="img/logo.png" alt="logo"/> 
			</div>
        </div>
    }
}

