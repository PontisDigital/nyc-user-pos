use gloo::net::http::Request;
use serde_json::json;
use stylist::{yew::styled_component, style};
use yew::prelude::*;
use yewdux::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::{sign_in::SignIn, pixel_rain::PixelRain};

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Debug)]
#[store(storage = "local", storage_tab_sync)]
pub struct CppOnboardingDay
{
	pub token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CppResponse
{
	pub token: Option<String>,
}

#[styled_component]
pub fn Home() -> Html
{
    let (user, dispatch) = use_store::<CppOnboardingDay>();
    let already_requested = use_state(|| false);
    if !*already_requested && user.token.is_none()
    {
        already_requested.set(true);
        wasm_bindgen_futures::spawn_local(async move
        {
            let response = Request::post("https://api.rainyday.deals/cpp_onboard")
                .json(&json!({}))
                .unwrap()
                .send()
                .await
                .unwrap()
                .json::<CppResponse>()
                .await
                .unwrap();

            if response.token.is_some()
            {
                dispatch.set(CppOnboardingDay { token: Some(response.token.unwrap()) });
            }
        });
    }
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
		margin-top: 20px;
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

	html!
	{
		<>
			<div class={root}>
				<div class={background}>
					<PixelRain />
				</div>
                <div class={save_10}>
                    <h1>{ "SAVE WHEN YOU SHOP IN BED-STUY" }</h1>
                </div>
                <div class={delis}>
                    <h2>{ "GREENE FOOD DELI" }</h2>
                    <h2>{ "GREENE MARKET DELI" }</h2>
                    <h2>{ "and more coming" }</h2>
                </div>
                <div class = {steps}>
                    <h1>{"Steps to save"}</h1>
                    <h2>{"1. Scan a QR code at a participating store"}</h2>
                    <img class = {step_img.clone()} src="img/step1.png" alt="step1"/> 
                    <h2>{"2. Enter the price the cashier is charging you"}</h2>
                    <img class = {step_img.clone()} src="img/step2.png" alt="step2"/> 
                    <h2>{"3. We bring the price down 10%"}</h2>
                    <img class = {step_img.clone()} src="img/step3.png" alt="step3"/> 
                </div>
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
