use yew::prelude::*;
use stylist::{yew::styled_component, style};

#[derive(Properties, PartialEq)]
pub struct Props
{
  pub title: Option<String>,
  pub on_click: Option<Callback<MouseEvent>>,
}

#[styled_component]
pub fn Button(props: &Props) -> Html
{
    let stylesheet = style!
    {
r#"
backface-visibility: hidden;
background: #332cf2;
border: 0;
border-radius: .375rem;
box-sizing: border-box;
color: #fff;
cursor: pointer;
display: inline-block;
font-family: Bai Jamjuree,sans-serif;
font-size: 1.5rem;
font-weight: 700;
letter-spacing: -.01em;
line-height: 1.3;
padding: 1rem 7rem;
position: relative;
text-align: left;
text-decoration: none;
transform: translateZ(0) scale(1);
transition: transform .2s;
user-select: none;
-webkit-user-select: none;
touch-action: manipulation;

&:disabled {
  color: #787878;
  cursor: auto;
}

&:not(:disabled):hover {
  transform: scale(1.05);
  @media (max-width: 1024px) {
    transform: translateY(0rem) scale(3.05);
  }
}

&:not(:disabled):hover:active {
  transform: scale(1.05) translateY(.125rem);
  @media (max-width: 1024px) {
    transform: translateY(.125rem) scale(3.05);
  }
}

&:focus {
  outline: 0 solid transparent;
}

&:focus:before {
  border-width: .125rem;
  content: "";
  left: calc(-1*.375rem);
  pointer-events: none;
  position: absolute;
  top: calc(-1*.375rem);
  transition: border-radius;
  user-select: none;
}

&:focus:not(:focus-visible) {
  outline: 0 solid transparent;
}

&:not(:disabled):active {
  transform: translateY(.125rem);
  @media (max-width: 1024px) {
    transform: translateY(3.125rem) scale(3);
  }
}

@media (max-width: 1024px) {
  font-size: 1rem;
  padding: .75rem 5rem;
  transform: translateZ(0) scale(3);
  margin-top: 100px;
}

"#

    }.unwrap();

    html!
    {
	  if props.on_click.is_none()
	  {
        <button class={stylesheet}>
		  {props.title.clone().unwrap_or("Submit".to_string())}
		</button>
	  }
	  else
	  {
        <button class={stylesheet} onclick={props.on_click.as_ref().unwrap()}>
		  {props.title.clone().unwrap_or("Submit".to_string())}
		</button>
	  }
    }
}

