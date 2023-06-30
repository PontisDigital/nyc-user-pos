use gloo::console::log;
use stylist::{yew::styled_component, style};
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct Props
{
	pub onchange: Option<Callback<String>>,
}

#[styled_component]
pub fn SaleInput(props: &Props) -> Html
{
	let stylesheet = style!(r#"

		input[type="text"] {
  width: 100%;
  font-size: 5em;
  text-align: center;
  color: #b3a8ff;
  padding-top: 20px;
  padding-bottom: 20px;
  display: block;
  background-color: rgba(255,255,255,0.2);
  margin-top: 40px;
  margin-left: auto;
  margin-right: auto;
  border-radius: 0px;
  border: none;
  outline: none;
  box-shadow: none;
}

input[type="text"]:focus {
  border-color: #000;
  box-shadow: none;
}

/* For mobile devices */

@media (max-width: 1024px) {
  input[type="text"] {
    width: 100%;
	font-size: 3em;
  }
}

/* Firefox */
input[type=text] {
  -moz-appearance: textfield;
}

input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

	"#).unwrap();

	let oninput = Callback::from(move |event: InputEvent|
		{
			event.prevent_default();
			let input_field = event
				.target()
				.unwrap()
				.unchecked_into::<HtmlInputElement>();
			let value = input_field.value();
			input_field.set_value(&format_code(value.as_str()));
		});

	let oninvalid = Callback::from(|event: Event|
		{
			let input_field = event
				.target()
				.unwrap()
				.unchecked_into::<HtmlInputElement>();
			log!(format!("Invalid input: {}", input_field.value()));
		});

	let onchange_callback = props.onchange.clone();
	let onchange = Callback::from(move |event: Event|
		{
			event.prevent_default();
			let value = event
				.target()
				.unwrap()
				.unchecked_into::<HtmlInputElement>()
				.value();
			if onchange_callback.is_some()
			{
				onchange_callback.as_ref().unwrap().emit(value);
			}
		});

	html!
	{
		<div class = {stylesheet}>
			<input type="text"
				inputmode="decimal"
				onchange={onchange}
				oninput={oninput}
				oninvalid={oninvalid}
				id="amount"
				name="amount"
				placeholder="$0.00"
				required=true
				/>
		</div>
	}
}

fn format_code(input: &str) -> String
{
	if input.len() == 0
	{
		return String::from("$");
	}
	if input[0..1].to_string() != "$"
	{
		return format!("${}", input);
	}
	format!("{}", input)
}
