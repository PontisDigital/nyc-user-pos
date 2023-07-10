use stylist::{yew::styled_component, style};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{components::{button::Button, sign_in::SignIn, pixel_rain::PixelRain}, pages::user_pos::AnonUserPersistentState};

#[styled_component]
pub fn Home() -> Html
{
	let join_hit_state = use_state(|| false);
	let jhs = join_hit_state.clone();
	let joinbuttonpressed = Callback::from(move |event: SubmitEvent|
		{
			event.prevent_default();
			jhs.set(true);
		});

	let auth = use_store::<AnonUserPersistentState>().0;

	let root = style!(r#"

		position: relative;
		margin: auto;
		width: 100%;
		text-align: center;

		"#).unwrap();


	let background = style!(r#"

		background-color: #000000;
		color: #ffffff;
		position: fixed;
		top: 0;
		left: 0;
		bottom: 0;
		right: 0;
		z-index: -1;

		"#).unwrap();

	let save_10 = style!(r#"

		font-family: 'Bai Jamjuree', sans-serif;
		font-size: 20px;
		font-weight: bold;
		margin-top: 20px;
		margin-bottom: 20px;
		color: #30F0CE;

		"#).unwrap();

	let delis = style!(r#"

		font-family: 'Bai Jamjuree', sans-serif;
		font-size: 20px;
		font-weight: bold;
		margin-top: 20px;
		margin-bottom: 20px;

		"#).unwrap();

	let step_img = style!(r#"
		width: 35%;
		height: auto;
		border: 5px solid #30F0CE;
		@media (max-width: 1024px)
		{
			width: 60%;
		}
	"#).unwrap();
	let steps = style!(r#"

		font-family: 'Bai Jamjuree', sans-serif;
		font-size: 20px;
		font-weight: bold;
		margin-top: 40px;
		margin-bottom: 20px;
		padding-left: 20px;
		padding-right: 20px;

		@media (max-width: 1024px) {
			margin-top: 80px;
			padding-top: 80px;
		}

		"#).unwrap();
	
	let footer_parent = style!(r#"

		position: relative;
		margin: auto;
		width: 100%;
		text-align: center;
		justify-content: center;
		font-family: 'Bai Jamjuree', sans-serif;
		font-size: 15px;
		font-weight: bold;
		margin-top: 40px;
		margin-bottom: 20px;

		@media (max-width: 1024px) {
			margin-top: 80px;
			padding-top: 80px;
		}

		"#).unwrap();
	let footer = style!(r#"
		position: relative;
		bottom: 0px;
		text-align: center;
		justify-content: center;
		"#).unwrap();

	let sign_on_message = style!(r#"

		font-family: 'Bai Jamjuree', sans-serif;
		font-size: 20px;
		font-weight: bold;
		margin-top: 40px;
		margin-bottom: 20px;
		color: #30F0CE;

		"#).unwrap();

	let input_style	= style!(r#"

		font-family: 'Bai Jamjuree', sans-serif;
		font-size: 20px;
		font-weight: bold;
		margin-top: 40px;
		margin-bottom: 20px;
		color: #30F0CE;

		"#).unwrap();

	html!
	{
		<>
			<div class={root}>
				<div class={background}>
					//<img src="img/logo.png" alt="logo"/> 
					<PixelRain />
				</div>
				if !*join_hit_state
				{
					<div class={save_10}>
						<h1>{ "SAVE WHEN YOU SHOP IN BED-STUY" }</h1>
					</div>
					<div class={delis}>
						<h2>{ "GREENE FOOD DELI" }</h2>
						<h2>{ "GREENE MARKET DELI" }</h2>
						<h2>{ "and more coming" }</h2>
					</div>
					<div class={sign_on_message}>
						if auth.token.is_none()
						{
							//<h2> { "One time sign up, instant savings" }</h2>
								/*
							<form onsubmit={joinbuttonpressed}>
								<Button title={"Join for Free"}/>
							</form>
								*/
						}
						else
						{
							//<h2> { "You're registered - Savings are just one tap away at checkout" }</h2>
						}
					</div>
					<div class = {steps}>
						<h1>{"Steps to save"}</h1>
						<h2>{"1. Scan a QR code at a participating store"}</h2>
						//<div class={img_border}>
							<img class = {step_img.clone()} src="img/step1.png" alt="step1"/> 
						//</div>
						<h2>{"2. Enter the price the cashier is charging you"}</h2>
						<img class = {step_img.clone()} src="img/step2.png" alt="step2"/> 
						<h2>{"3. We bring the price down 10%"}</h2>
						<img class = {step_img.clone()} src="img/step3.png" alt="step3"/> 
					</div>
				}
				else
				{
					<div class={input_style}>
						<h2>{ "Enter your phone number" }</h2>
						<h2>{ "Start saving instantly" }</h2>
						<SignIn />
					</div>
				}
				<div class = {footer_parent}>
					<div class = {footer}>
						<p> { "rainyday" } </p>
						<p> { "never pay full price again" } </p>
					</div>
				</div>
			</div>
		</>
	}
}
