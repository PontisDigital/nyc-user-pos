use gloo::console::log;
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

		input[type="tel"] {
  width: 100%;
  font-size: 5em;
  text-align: center;
  background-color: rgba(0, 0, 0, 0.7);
  color: #b3a8ff;
  padding-top: 20px;
  padding-bottom: 20px;
  display: block;
  margin-top: 40px;
  margin-left: auto;
  margin-right: auto;
  border-radius: 0px;
  border: none;
  outline: none;
  box-shadow: none;
}

input[type="tel"]:focus {
  border-color: #000;
  box-shadow: none;
}

/* For mobile devices */

@media (max-width: 1024px) {
  input[type="tel"] {
    width: 100%;
	font-size: 3em;
  }
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

	let oninvalid = Callback::from(|event: Event|
		{
			let input_field = event
				.target()
				.unwrap()
				.unchecked_into::<HtmlInputElement>();
			log!(format!("Invalid input: {}", input_field.value()));
		});

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
			<input type="tel"
				onchange={onchange}
				oninput={oninput}
				oninvalid={oninvalid}
				id="phone"
				name="phone"
				pattern="\\(\\d{3}\\)-\\d{3}-\\d{4}"
				placeholder="(123)-456-7890"
				required=true
				autofocus=true
				title="Please enter a valid phone number"
				/>
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

