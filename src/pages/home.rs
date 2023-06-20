use stylist::{yew::styled_component, style};
use yew::prelude::*;

#[styled_component]
pub fn Home() -> Html
{
    let stylesheet = style!(
        r#"
            color: white;
            font-size: 18px;
            text-align: center;
            font-family: Bai Jamjuree, Courier, monospace;
            letter-spacing: 0.1em;
            position: absolute;
            left: 50%;
            top: 50%;
            -webkit-transform: translate(-50%, -50%);
            transform: translate(-50%, -50%);
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
            top: 85%;
            -webkit-transform: translate(-50%, -85%);
            transform: translate(-50%, -85%);
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
        "#
        ).unwrap();

    html!
    {
        <>
            <div class={stylesheet}>
                <h2>{ "GREENE FOOD DELI" }</h2>
                <h2>{ "GREENE MARKET DELI" }</h2>
                <h2>{ "FRANKLIN CONVENIENCE" }</h2>
            </div>
            <div class={thing_at_bottom}>
                <p> { "rainyday" } </p>
                <p> { "Thank us later" } </p>
            </div>
        </>
    }
}
