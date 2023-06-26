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
pub fn PasswordInput(props: &Props) -> Html
{
	let stylesheet = style!(r#"

		input[type="password"] {
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

input[type="password"]:focus {
  border-color: #000;
  box-shadow: none;
}

/* For mobile devices */

@media (max-width: 1024px) {
  input[type="password"] {
    width: 100%;
	font-size: 3em;
  }
}

/* Firefox */
input[type=password] {
  -moz-appearance: textfield;
}

input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

	"#).unwrap();

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
			<input type="password"
				onchange={onchange}
				oninvalid={oninvalid}
				id="password"
				name="password"
				placeholder="Enter your password"
				required=true
				/>
		</div>
	}
}
