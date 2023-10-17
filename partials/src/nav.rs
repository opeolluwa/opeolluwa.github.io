use super::logo::AppLogo;
use leptos::*;
#[component]
pub fn AppNavigation() -> impl IntoView {
    let routes: [&str; 3] = ["Home", "Contact", "About"];
    view! {
        <nav>
            <AppLogo/>

            <ul>{routes.into_iter().map(|route| view! { <li>{route}</li> }).collect_view()}</ul>

        // www.w3.org/2000/svg"

        </nav>
    }
}
