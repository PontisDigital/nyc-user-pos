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

	/* Chrome, Safari, Edge, Opera */
	input::-webkit-outer-spin-button,
	input::-webkit-inner-spin-button {
	  -webkit-appearance: none;
	  margin: 0;
	}

	/* Firefox */
	input[type=number] {
	  -moz-appearance: textfield;
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
