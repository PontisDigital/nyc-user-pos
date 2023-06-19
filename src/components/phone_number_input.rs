use stylist::{yew::styled_component, style};
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct Props
{
	pub onchange: Callback<String>
}

#[styled_component]
pub fn PhoneInput(props: &Props) -> Html
{
	let stylesheet = style!(r#"

	input {
		display: block;
		margin: 2em auto;
		border: none;
		padding: 16px;
		width: calc((14 * 1ch) + (13 * 0.5ch)); /* Adjusted width for 14 characters */
		background: repeating-linear-gradient(90deg,
			dimgrey 0, dimgrey 1ch,
			transparent 0, transparent 0.5ch)
			0 100%/calc((14 * 1ch) + (14 * 0.5ch) - 0.5ch) 2px no-repeat;
		font: 5ch Bai Jamjuree, consolas, monospace;
		letter-spacing: 0.5ch;
		text-align: center;
		color: #9991ef;
	}

	input:focus {
		outline: none;
	}


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

	let input_callback = props.onchange.clone();

	let onchange = Callback::from(move |event: Event|
		{
			let value = event
				.target()
				.unwrap()
				.unchecked_into::<HtmlInputElement>()
				.value();
			input_callback.emit(value);
		});

	html!
	{
		<div class = {stylesheet}>
			<input type="tel" onchange={onchange} oninput={oninput} id="phone" name="phone" pattern="[0-9]{3}-[0-9]{3}-[0-9]{4}" placeholder="(123)-456-7890" required=true />
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

