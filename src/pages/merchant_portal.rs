use std::thread::sleep;

use gloo::{net::http::Request, timers::callback::Timeout};
use stylist::{yew::styled_component, style};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props
{
    pub merchant_uid: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PollResponse
{
	message: String,
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

	let random_message = use_state(|| "".to_string());
	let random_message_clone = random_message.clone();

	Timeout::new(1_000, move ||
	{
		wasm_bindgen_futures::spawn_local(async move
		{
			let req = Request::post("https://api.rainyday.deals/merchant").send().await;
			match req
			{
				Ok(res) =>
				{
					let message = res.json::<PollResponse>().await.unwrap();
					random_message_clone.set(message.message);
				},
				Err(_) =>
				{
				}
			}
		});
	}).forget();

    html! {
        <div class={stylesheet}>
            <h1>{ "Merchant Portal" }</h1>
            <h1>{ "You'll be notified here when a customer scans your QR Code" }</h1>
			<p>{ format!("Poll Test: {}", *random_message) }</p>
            <p>{ format!("Merchant UID: {}", props.merchant_uid) }</p>
			<div class={img_style}>
				<img src="img/logo.png" alt="logo"/> 
			</div>
        </div>
    }
}

