use gloo::{console::log, net::http::Request};
use rust_decimal::Decimal;
use rusty_money::{Money, iso};
use wasm_bindgen::JsCast;
use web_sys::{HtmlFormElement, HtmlInputElement};
use yew::prelude::*;
use stylist::{yew::styled_component, style};
use yewdux::prelude::*;

use crate::{components::{sale_amount_input::SaleInput, button::Button}, pages::user_pos::{UserPersistentState, Merchant}};

#[derive(serde::Serialize, serde::Deserialize)]
struct RequestSent
{
	success: bool,
	error: Option<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props
{
	pub merchant: Merchant,
}

#[styled_component]
pub fn UserPriceForm(props: &Props) -> Html
{
	let user_data = use_store::<UserPersistentState>();
	let merchant_uid = props.merchant.uid.clone();
	let purchase_complete = use_state(|| false);
	let pc = purchase_complete.clone();
	let discount_state = use_state(|| "".to_string());
	let ds = discount_state.clone();
	let on_submit = Callback::from(move |event: SubmitEvent|
	{
		event.prevent_default();

		let discount_state = ds.clone();
		let purchase_complete = pc.clone();
		let merchant_uid = merchant_uid.clone();
		let user_data = user_data.clone();
		let target = event.target().unwrap();
		let form = target.dyn_into::<HtmlFormElement>().unwrap();
		let input = form.elements().named_item("amount").unwrap().dyn_into::<HtmlInputElement>().unwrap().value();
		let mut input: String = input[1..].to_string();
		if input.find(".").is_some()
		{
			if input.find(".").unwrap() == 0
			{
				input.insert_str(0, "0");
			}
		}
		let money = Money::from_str(&input, iso::USD).unwrap();
		let with_discount = Money::from_decimal(
			money.amount().checked_mul(Decimal::from_str_exact("0.9").unwrap()).unwrap()
			, iso::USD);
		let mut with_discount_str: String = format!("{}", with_discount);
		if with_discount_str.find(".").unwrap_or(with_discount_str.len()) == with_discount_str.len() - 2
		{
			with_discount_str.push_str("0");
		}
		let mut money_str: String = format!("{}", money);
		if money_str.find(".").unwrap_or(money_str.len()) == money_str.len() - 2
		{
			money_str.push_str("0");
		}

		if money_str.is_empty() || money_str.len() < 2
		{
			return;
		}

		log!(format!("Money: {}", money_str));
		log!(format!("With discount: {}", with_discount_str));

		wasm_bindgen_futures::spawn_local(async move
			{
				let response = Request::post("https://api.rainyday.deals/merchant")
					.json(&serde_json::json!(
							{
								"request_type": "user_submission",
								"merchant_uid": merchant_uid,
								"user_token": user_data.0.token.as_ref().unwrap().to_string(),
								"user_phone": user_data.0.phone,
								"price_of_sale": money_str,
								"price_with_discount": with_discount_str,
							}
					))
					.unwrap()
					.send()
					.await
					.unwrap()
					.json::<RequestSent>()
					.await
					.unwrap();
				purchase_complete.set(response.success);
				if response.success
				{
					discount_state.set(with_discount_str);
				}
			});
	});
	let border = style!(r#"
		margin-left: 100px;
		margin-right: 100px;
		margin-bottom: 48px;
		"#).unwrap();
	html!
	(
	<>
		if !*purchase_complete
		{
			<form onsubmit={on_submit}>
				<div class={border}>
					<SaleInput />
				</div>
				<Button title="Submit" />
			</form>
		}
		else
		{
			<h1>{format!("You owe {} {}", props.merchant.name, *discount_state)}</h1>
		}
	</>
	)
}
