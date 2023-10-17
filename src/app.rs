use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use partials::{footer::AppFooter, nav::AppNavigation};

#[component]

pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/opeolluwa.css"/>

        // import icons library
        <Link
            rel="stylesheet"
            href="https://cdn.jsdelivr.net/gh/devicons/devicon@v2.15.1/devicon.min.css"
        />

        // favicon
        <Link rel="apple-touch-icon" sizes="180x180" href="assets/favicon/apple-touch-icon.png"/>
        <Link rel="icon" type_="image/png" sizes="32x32" href="assets/favicon/favicon-32x32.png"/>
        <Link rel="icon" type_="image/png" sizes="16x16" href="assets/favicon/favicon-16x16.png"/>
        <Link rel="manifest" href="src/assets/favicon/site.webmanifest"/>

        // sets the document title
        <Title text="Adeoye Adefemi"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <AppNavigation/>
        <header>
            <div>
                <h1 class="text-5xl">Hey there r#"👋"# <br/> r#"I'm"# <span>Opeoluwa</span> ,</h1>
                <p class="leading-1 mt-4 hidden">
                    r#"a software developer driven by need for optimized solution. I take
                    interest in scaffolding application's backend whilst aiming at
                    becoming an embedded systems engineer."#
                </p>
            </div>
            <div>

                <img src="./public/images/opeolluwa.jpg" alt="Hero Image"/>
            </div>
        </header>
        <main >

        </main>
        <AppFooter/>
    }
}
