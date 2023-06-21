use std::collections::HashMap;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component, style};
use yew::prelude::*;

use crate::components::user_pos_form::UserPOSForm;

#[derive(Properties, PartialEq)]
pub struct Properties
{
	pub id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Merchant
{
	pub uid: String,
	pub name: String,
}

#[styled_component]
pub fn UserPOS(props: &Properties) -> Html
{
	let heading = style!(r#"

		  display: flex;
		  justify-content: center;
		  align-items: center;

	"#).unwrap();

	let message = style!(r#"
		font-size: 48px;
		font-weight: 400;
		margin-top: 0px;
		margin-bottom: 0px;
		@media (max-height: 1080px) {
			font-size: 30px;
		}
	"#).unwrap();

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

	let img = style!(r#"
		img {
			margin-top: 100px;
			width: 20%;
			height: auto;
			@media (max-height: 1080px) {
				width: 5%;
			}
		}
		"#).unwrap();

	let bottom = style!(r#"
		
		"#).unwrap();

	let has_loaded = use_state(|| false);
	let merchant = use_state(|| Merchant { uid: props.id.clone(), name: "Loading...".to_string() });
	let uid = props.id.clone();
	let callback = {
		let state = merchant.clone();
		Callback::from(move |merchant: Merchant| state.set(merchant))
	};

	if !(*has_loaded)
	{
		get_merchant_map(uid, callback);
		has_loaded.set(true);
	}

	html!
	{
		<div class={stylesheet}>
			if merchant.name != "Loading..."
			{
				<div class={heading}>
					<h1>{format!("rainyday x {}", merchant.name)}</h1>
				</div>
				<div class={message}>
					<h2>{ "one-time signup." }</h2>
					<h2>{ "*10%* off everything forever." }</h2>
				</div>
				<UserPOSForm />
				<div class={img}>
					<img src="img/logo.png" alt="logo"/> 
				</div>
				<div class={bottom}>
					<h3>{ "Find us all over Bed-Stuy" }</h3>
				</div>
			}
			else
			{
				<h1>{ "Loading..." }</h1>
			}
		</div>
	}
}

fn get_merchant_map(uid: String, callback: Callback<Merchant>)
{
	wasm_bindgen_futures::spawn_local(async move
		{
			let name = get_merchant_name(uid.clone()).await;
			let merchant = Merchant
			{
				name,
				uid,
			};
			callback.emit(merchant);
		});
}

async fn get_merchant_name(uid: String) -> String
{
	let merchant_map = Request::get("https://raw.githubusercontent.com/PontisDigital/nyc-user-pos/master/merchants.json")
		.send()
		.await
		.unwrap()
		.json::<HashMap<String, Merchant>>()
		.await
		.unwrap();
	merchant_map.get(&uid).unwrap().name.clone()
}

