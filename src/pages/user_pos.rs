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
	let stylesheet = style!(r#"

		font-family: 'Bai Jamjuree', sans-serif;
		text-align: center;

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
				<h1>{ format!("Welcome to {}", merchant.name)}</h1>
				<UserPOSForm />
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

