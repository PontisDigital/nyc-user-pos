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
    #[at("/pos/:merchant_uid")]
    UserPOS { merchant_uid: String },
}

pub fn switch(routes: Route) -> Html
{
    match routes
    {
        Route::Home => html! { <Home /> },
        Route::Discounts => html! { <Discounts /> },
        Route::UserPOS { merchant_uid } => html! { <UserPOS merchant_uid={merchant_uid}/> },
    }
}

