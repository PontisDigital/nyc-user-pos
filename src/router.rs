use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, discounts::Discounts, user_pos::UserPOS, merchant_portal::MerchantPortal};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route
{
    #[at("/")]
    Home,
    #[at("/discounts")]
    Discounts,
    #[at("/pos/:merchant_uid")]
    UserPOS { merchant_uid: String },
    #[at("/merchant/:merchant_uid")]
    Merchant { merchant_uid: String },
}

pub fn switch(routes: Route) -> Html
{
    match routes
    {
        Route::Home => html! { <Home /> },
        Route::Discounts => html! { <Discounts /> },
        Route::UserPOS { merchant_uid } => html! { <UserPOS merchant_uid={merchant_uid}/> },
        Route::Merchant { merchant_uid } => html! { <MerchantPortal merchant_uid={merchant_uid}/> },
    }
}

