use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, discounts::Discounts, user_pos::UserPOS};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route
{
    #[at("/")]
    Home,
    #[at("/discounts")]
    Discounts,
    #[at("/pos/:id")]
    UserPOS { id: String },
}

pub fn switch(routes: Route) -> Html
{
    match routes
    {
        Route::Home => html! { <Home /> },
        Route::Discounts => html! { <Discounts /> },
        Route::UserPOS { id } => html! { <UserPOS id={id}/> },
    }
}

