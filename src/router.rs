use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, discounts::Discounts};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route
{
    #[at("/")]
    Home,
    #[at("/discounts")]
    Discounts,
}

pub fn switch(routes: Route) -> Html
{
    match routes
    {
        Route::Home => html! { <Home /> },
        Route::Discounts => html! { <Discounts /> },
    }
}

