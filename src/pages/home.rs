use stylist::{yew::styled_component, style};
use yew::prelude::*;

#[styled_component]
pub fn Home() -> Html
{
    let thing_at_top = style!(
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
                    font-size: 72px;
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
            @media (max-width: 1024px) {
                & p {
                    font-size: 40px;
                    font-weight: 400;
                    margin: 0;
                    padding: 0;
                    letter-spacing: 0.1em;
                }
            }
        "#
        ).unwrap();

    html!
    {
        <>
            <div class={thing_at_top}>
                <img src="img/logo.png" alt="logo"/> 
            </div>
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
