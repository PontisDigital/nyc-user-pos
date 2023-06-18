use yew::prelude::*;

#[function_component]
pub fn Home() -> Html
{
    html!
    {
        <div>
            <h1>{ "Hello, NYC!" }</h1>
            <p>{ "Coming to Bed Stuy real soon!" }</p>
        </div>
    }
}
