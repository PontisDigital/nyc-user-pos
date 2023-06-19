mod router;
mod pages;
mod components;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component]
pub fn App() -> Html
{
    html!
    {
        <BrowserRouter>
            <Switch<Route> render={router::switch} />
        </BrowserRouter>
    }
}
