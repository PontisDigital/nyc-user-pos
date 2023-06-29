use stylist::{yew::styled_component, style};
use yew::prelude::*;

use crate::components::user_pos_form::UserPOSForm;

#[styled_component]
pub fn SignIn() -> Html
{

	let stylesheet = style!(r#"

		font-family: 'Bai Jamjuree', sans-serif;
		text-align: center;
		height: 100vh;
		z-index: 2;

		@media (max-width: 1024px) {
			font-size: 30px;
		}

		@media (max-width: 480px) {
		}

		@media (max-height: 1080px) {
			height: 50vh;
			font-size: 20px;
		}

	"#).unwrap();

	html!(
	  <>
		<div class={stylesheet}>
		  <UserPOSForm />
		</div>
	  </>
	  )
}
