use super::logo::AppLogo;
use leptos::*;
#[component]
pub fn AppNavigation() -> impl IntoView {
    let routes: [&str; 3] = ["Home", "Contact", "About"];
    view! {
        <nav class="view no-mb">
            <AppLogo/>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                width="24"
                height="24"
                class="d-none"
            >
                <path fill="none" d="M0 0h24v24H0z"></path>
                <path
                    d="M18 18v2H6v-2h12zm3-7v2H3v-2h18zm-3-7v2H6V4h12z"
                    fill="currentColor"
                ></path>
            </svg>
            <ul>{routes.into_iter().map(|route| view! { <li>{route}</li> }).collect_view()}</ul>
        </nav>
    }
}
