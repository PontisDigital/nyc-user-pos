use std::collections::HashMap;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component, style};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::{user_pos_form::UserPOSForm, logged_in_pos::LoggedInPOS};

#[derive(Properties, PartialEq)]
pub struct Properties
{
	pub merchant_uid: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Merchant
{
	pub uid: String,
	pub name: String,
}

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Debug)]
#[store(storage = "local", storage_tab_sync)]
pub struct UserPersistentState
{
	pub token: Option<String>,
	pub phone: Option<String>,
}

#[styled_component]
pub fn UserPOS(props: &Properties) -> Html
{
	let auth = use_store::<UserPersistentState>().0;

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
	let merchant = use_state(|| Merchant { uid: props.merchant_uid.clone(), name: "Loading...".to_string() });
	let uid = props.merchant_uid.clone();
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
				<div class={heading.clone()}>
					<h1>{format!("rainyday x {}", merchant.name)}</h1>
				</div>

				if auth.token.is_none()
				{
					<div class={message}>
						<h2>{ "Enter phone number" }</h2>
						<h2>{ "Start saving now" }</h2>
					</div>
					<UserPOSForm />
				}
				else
				{
					<LoggedInPOS merchant={(*merchant).clone()}/>
				}
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

pub fn get_merchant_map(uid: String, callback: Callback<Merchant>)
{
	wasm_bindgen_futures::spawn_local(async move
		{
			let merchant = get_merchant(uid.clone()).await;
			callback.emit(merchant);
		});
}

pub async fn get_merchant(uid: String) -> Merchant
{
	let response = Request::get("https://raw.githubusercontent.com/PontisDigital/nyc-user-pos/master/merchants.json")
		.send()
		.await
		.unwrap()
		.json::<HashMap<String,Merchant>>()
		.await
		.unwrap();
	response.get(&uid).unwrap().clone()
}

