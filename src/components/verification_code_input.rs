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
pub fn CodeInput(props: &Props) -> Html
{
	let stylesheet = style!(r#"

		input[type="number"] {
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

input[type="number"]:focus {
  border-color: #000;
  box-shadow: none;
}

/* For mobile devices */

@media (max-width: 1024px) {
  input[type="number"] {
    width: 100%;
	font-size: 3em;
  }
}

/* Firefox */
input[type=number] {
  -moz-appearance: textfield;
}

input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

	"#).unwrap();

	let oninput = Callback::from(|event: InputEvent|
		{
			let input_field = event
				.target()
				.unwrap()
				.unchecked_into::<HtmlInputElement>();
			let value = input_field.value();
			input_field.set_value(&format_code(value.as_str()));
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
			<input type="number"
				onchange={onchange}
				oninput={oninput}
				oninvalid={oninvalid}
				id="code"
				name="code"
				pattern="\\d{6}"
				placeholder="123456"
				required=true
				/>
		</div>
	}
}

fn format_code(input: &str) -> String
{
	// Remove any non-digit characters from the input
	let digits: String = input.chars().filter(|c| c.is_digit(10)).collect();

	if digits.len() > 6
	{
		// If the input is longer than 6 digits, truncate it
		return digits[0..6].to_string();
	}
	digits
}
