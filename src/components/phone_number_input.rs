use stylist::{yew::styled_component, style};
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[styled_component]
pub fn PhoneInput() -> Html
{
	let stylesheet = style!(r#"

		"#).unwrap();

	let oninput = Callback::from(|event: InputEvent|
		{
			let input_field = event
				.target()
				.unwrap()
				.unchecked_into::<HtmlInputElement>();
			let value = input_field.value();
			input_field.set_value(&format_phone_number(value.as_str()));
		});

	html!
	{
		<div class = {stylesheet}>
			<input type="tel" oninput={oninput} id="phone" name="phone" pattern="[0-9]{3}-[0-9]{3}-[0-9]{4}" placeholder="(123)-456-7890" required=true />
		</div>
	}
}

fn format_phone_number(input: &str) -> String
{
	let cleaned_input: String = input
		.chars()
		.filter(|c| c.is_digit(10))
		.collect();

	if cleaned_input.len() >= 7 && cleaned_input.len() < 11
	{
		let area_code = &cleaned_input[0..3];
		let prefix = &cleaned_input[3..6];
		let line_number = &cleaned_input[6..];
		format!("({})-{}-{}", area_code, prefix, line_number)
	}
	else if cleaned_input.len() < 4
	{
		if !cleaned_input.is_empty()
		{
			format!("({}", cleaned_input)
		}
		else
		{
			"".to_owned()
		}
	}
	else if cleaned_input.len() < 7
	{
		let area_code = &cleaned_input[0..3];
		let prefix = &cleaned_input[3..];
		format!("({})-{}", area_code, prefix)
	}
	else
	{
		let area_code = &cleaned_input[0..3];
		let prefix = &cleaned_input[3..6];
		let line_number = &cleaned_input[6..10];
		format!("({})-{}-{}", area_code, prefix, line_number)
	}
}

