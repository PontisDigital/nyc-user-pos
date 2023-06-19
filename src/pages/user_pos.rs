use std::collections::HashMap;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component, style};
use yew::prelude::*;

use crate::components::button::Button;

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
	let stylesheet = style!(r#"
		font-family: 'Bai Jamjuree', sans-serif;
		text-align: center;
		"#).unwrap();
	let has_loaded = use_state(|| false);
	let current_merchant = use_state(|| Merchant { uid: props.id.clone(), name: "Loading...".to_string() });
	let uid = props.id.clone();
	let callback = {
		let state = current_merchant.clone();
		Callback::from(move |merchant: Merchant| state.set(merchant))
	};

	if !(*has_loaded)
	{
		wasm_bindgen_futures::spawn_local(async move
			{
				let merchant_map = Request::get("https://raw.githubusercontent.com/PontisDigital/nyc-user-pos/master/merchants.json")
					.send()
					.await
					.unwrap()
					.json::<HashMap<String, Merchant>>()
					.await
					.unwrap();
				let merchant = Merchant
				{
					name: merchant_map.get(&uid).unwrap().name.clone(),
					uid,
				};
				callback.emit(merchant);
			});
		has_loaded.set(true);
	}

	let merchant_name = &current_merchant.name;

	html!
	{
		<div class={stylesheet}>
			if merchant_name != "Loading..."
			{
				<h1>{ format!("Welcome to {}", merchant_name)}</h1>
				<h2>{ "User POS" }</h2>
				<Button />
			}
			else
			{
				<h1>{ "Loading..." }</h1>
			}
		</div>
	}
}

