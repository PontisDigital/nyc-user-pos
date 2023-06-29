use stylist::{yew::styled_component, style};
use yew::prelude::*;

use crate::components::button::Button;

#[styled_component]
pub fn Home() -> Html
{
	let image_in_back = style!(
		r#"
			position: absolute;
			left: 50%;
			top: 50%;
			-webkit-transform: translate(-50%, -50%);
			transform: translate(-50%, -50%);
			opacity: 0.25;
			& img {
				width: 425px;
				height: auto;
			}
			@media (max-width: 1024px) {
				& img {
					width: 1000px;
					height: auto;
				}
			}
			@media (max-width: 480px) {
				& img {
					width: 300px;
					height: auto;
				}
			}
		"#
		).unwrap();

	let stylesheet = style!(
		r#"
			color: white;
			text-align: center;
			font-family: Bai Jamjuree, Courier, monospace;
			letter-spacing: 0.1em;
			position: absolute;
			left: 50%;
			top: 50%;
			-webkit-transform: translate(-50%, -50%);
			transform: translate(-50%, -50%);
			@media (min-width: 1024px) {
				& h2 {
					font-size: 20px;
				}
				& p {
					font-size: 18px;
					font-weight: 400;
					margin: 0;
					padding: 0;
					letter-spacing: 0.1em;
				}
			}
			@media (max-width: 1024px) {
				& h2 {
					font-size: 58px;
					white-space: nowrap;
				}
			}
		"#
		).unwrap();

	let thing_at_bottom = style!(
		r#"
			color: white;
			font-size: 18px;
			text-align: center;
			font-family: Bai Jamjuree, Courier, monospace;
			letter-spacing: 0.1em;
			position: absolute;
			left: 50%;
			top: 90%;
			-webkit-transform: translate(-50%, -90%);
			transform: translate(-50%, -90%);
			& h2 {
				font-size: 20px;
			}
			& p {
				font-size: 18px;
				font-weight: 400;
				margin: 0;
				padding: 0;
				letter-spacing: 0.1em;
			}
			@media (max-width: 1024px) {
				& p {
					font-size: 40px;
					font-weight: 400;
					margin-top: 20px;
					padding: 0;
					letter-spacing: 0.1em;
				}
			}
		"#
		).unwrap();

		let save_10 = style!(
		r#"
			color: #30F0CE;
			text-align: center;
			font-family: Bai Jamjuree, Courier, monospace;
			letter-spacing: 0.1em;
			position: absolute;
			left: 50%;
			top: 25%;
			-webkit-transform: translate(-50%, -25%);
			transform: translate(-50%, -25%);
			@media (min-width: 1024px) {
				& h2 {
					font-size: 20px;
				}
				& p {
					font-size: 18px;
					font-weight: 400;
					margin: 0;
					padding: 0;
					letter-spacing: 0.1em;
				}
			}
			@media (max-width: 1024px) {
				font-size: 2.25vw;
				left: 50%;
				top: 10%;
				-webkit-transform: translate(-50%, -10%);
				transform: translate(-50%, -10%);
				white-space: nowrap;
				& h1 {
				}
			}
		"#
			).unwrap();

		let signupstyle = style!(r#"
			color: #30F0CE;
			text-align: center;
			font-family: Bai Jamjuree, Courier, monospace;
			letter-spacing: 0.1em;
			position: absolute;
			left: 50%;
			top: 75%;
			-webkit-transform: translate(-50%, -75%);
			transform: translate(-50%, -75%);
			@media (min-width: 1024px) {
				& h2 {
					font-size: 20px;
				}
				& p {
					font-size: 18px;
					font-weight: 400;
					margin: 0;
					padding: 0;
					letter-spacing: 0.1em;
				}
			}
			@media (max-width: 1024px) {
				font-size: 2.25vw;
				left: 50%;
				top: 10%;
				-webkit-transform: translate(-50%, -10%);
				transform: translate(-50%, -10%);
				white-space: nowrap;
				& h1 {
				}
			}
			"#).unwrap();
	let joinbuttonpressed = Callback::from(move |event: SubmitEvent|
		{
		});
	html!
	{
		<>
			<div class={save_10}>
				<h1>{ "SAVE AUTOMATICALLY WHEN YOU SHOP IN BED-STUY" }</h1>
			</div>
			<div class={image_in_back}>
				<img src="img/logo.png" alt="logo"/> 
			</div>
			<div class={stylesheet}>
				<h2>{ "GREENE FOOD DELI" }</h2>
				<h2>{ "GREENE MARKET DELI" }</h2>
				<h2>{ "FRANKLIN CONVENIENCE" }</h2>
				<h2>{ "and more coming" }</h2>
			</div>
			<div class={signupstyle}>
				<h2> { "One time sign up, instant savings" }</h2>
				<form onsubmit={joinbuttonpressed}>
					<Button title={"Join for Free"}/>
				</form>
			</div>
			<div class={thing_at_bottom}>
				<p> { "rainyday" } </p>
				<p> { "never pay full price again" } </p>
			</div>
		</>
	}
}
