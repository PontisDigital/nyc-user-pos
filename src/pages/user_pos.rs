use std::{collections::HashMap, borrow::Borrow};

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

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

#[function_component]
pub fn UserPOS(props: &Properties) -> Html
{
	let has_loaded = use_state(|| false);
	let state = use_state(|| Merchant { uid: props.id.clone(), name: "Loading...".to_string() });
	let uid = props.id.clone();
	let callback = {
		let state = state.clone();
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
					uid: uid.clone(),
					name: merchant_map.get(&uid).unwrap().name.clone(),
				};
				callback.emit(merchant);
			});
		has_loaded.set(true);
	}

	let merchant_name = state.borrow().name.clone();
	if merchant_name == "Loading..."
	{
		html!
		{
			<div class="user-pos">
				<h1>{ "Loading..." }</h1>
				<h2>{ "Loading..." }</h2>
			</div>
		}
	}
	else
	{
		html!
		{
			<div class="user-pos">
				<h1>{ format!("Welcome to {}", merchant_name)}</h1>
				<h2>{ "User POS" }</h2>
			</div>
		}
	}
}

