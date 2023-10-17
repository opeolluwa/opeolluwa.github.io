use leptos::*;

#[component]
pub fn AppFooter() -> impl IntoView {
    view! {
        <footer class="bg-yellow-500 px-[100px]">
            <div>
                <div class="social">
                    <a href="https://github.com/opeolluwa" target="_blank">
                        <i class="devicon-github-plain"></i>
                    </a>
                    <a href="https://twitter.com/_opeolluwa" target="_blank">
                        <i class="devicon-twitter-plain"></i>
                    </a>
                    <a href="https://www.linkedin.com/in/adefemi-adeoye/" target="_blank">
                        <i class="devicon-linkedin-plain"></i>
                    </a>
                </div>

                &copy; 2023 Opeoluwa - All Rights Reserved
            </div>
        </footer>
    }
}
