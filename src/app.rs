use crate::{
    app::components::app_navigation::AppNavigation,
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod partials;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/opeolluwa.css"/>

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
    view! { <main>
       <AppNavigation/>
       
      </main> }
}

//  <AppNavigation class="view no-mb" />
//       <div id="hero" class="view">
//         <!--hero text-->
//         <div>
//           <h1>
//             Hey there 👋 <br />
//             I&apos;m <span class="emphasis">Opeoluwa</span>,
//           </h1>
//           <p>
//             a software developer driven by need for optimized solution. I take
//             interest in scaffolding application&apos;s backend whilst aiming at
//             becoming an embedded systems engineer.
//           </p>
//           <div>
//             <router-link :to="{ name: 'projects' }">Explore Projects
//               <IconArrowRight />
//             </router-link>
//             <!-- <a href="#">Download CV</a> -->
//           </div>
//         </div>
//         <!--hero illustration-->
//         <div></div>
//       </div>

//       <!--recent section-->
//       <div id="projects" class="view">
//         <h2>
//           Checkout my <br /><span class="emphasis capitalize">recent Projects</span>
//         </h2>
//         <Carousel :items-to-show="1.25" :wrap-around="true" :breakpoints="breakpoints">
//           <Slide v-for="slide in 10" :key="slide">
//             <ProjectCard :title="''" :description="''" />
//           </Slide>

//           <template #addons> </template>
//         </Carousel>
//         <router-link :to="{ name: 'projects' }" class="emphasis">view more
//           <IconRight />
//         </router-link>
//       </div>

//       <!------------------------------my awesome technology stack---------------------->
//       <div id="tech-stack" class="view">
//         <h2>
//           My
//           <span class="emphasis capitalize">core technology</span> <br />
//           Stack 🚀
//         </h2>
//         <!--project card container-->
//         <div>
//           <div>
//             <TechCard v-for="(item, index) in stack" :colored="'colored'" :icon="item.icon" :key="index" />
//           </div>
//           <div id="tech-info">
//             <h3>Technology name</h3>
//             <p>
//               technology description Lorem ipsum dolor, sit amet consectetur adipisicing elit. Vel nobis doloremque hic
//               inventore sint explicabo deserunt
//               putting except
//             </p>
//             <h4>Projects</h4>
//             <ul>
//               <li>project one</li>
//               <li>project one</li>
//               <li>project one</li>
//               <li>project one</li>
//             </ul>
//           </div>
//         </div>
//       </div>

//       <!-------------------------------contact me--------------------------->
//       <div id="contact-me" class="view">
//         <div>
//           <h2>
//             Send a <br /><span class="emphasis capitalize">Quick</span> message
//           </h2>
//           <p>Let&apos;s talk about everything</p>
//           <button @click="goToContactPage">
//             Send Email
//             <IconCommunity />
//           </button>
//         </div>
//         <div>

//         </div>
//       </div>
