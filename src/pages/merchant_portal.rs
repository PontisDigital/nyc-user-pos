use gloo::{net::http::Request, timers::callback::Timeout};
use rust_decimal::Decimal;
use stylist::{yew::styled_component, style};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props
{
    pub merchant_uid: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
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
				<p>{ format!("Pending Sales: {:?}", (*pending_sales_state_clone)) }</p>
			}
			<div class={img_style}>
				<img src="img/logo.png" alt="logo"/> 
			</div>
        </div>
    }
}

