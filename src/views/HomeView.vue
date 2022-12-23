<script lang="ts">
//import components
import AppNavigation from "@/components/AppNavigation.vue";
import IconArrowRight from "@/components/icons/IconRight.vue";
import AppFooter from "@/components/AppFooter.vue";
import TechCard from "@/components/cards/TechCard.vue";
import ProjectCard from "@/components/cards/ProjectCard.vue";
import IconCommunity from "@/components/icons/IconCommunity.vue";
import "vue3-carousel/dist/carousel.css";
import { Carousel, Slide } from "vue3-carousel";
// import VueFinalModal from "vue-final-modal";
import IconRight from "@/components/icons/IconRight.vue";
import { defineComponent } from "vue";
import BaseTextInput from "@/components/BaseTextInput.vue";
export default defineComponent({
  name: "HomeView",
  components: {
    AppNavigation,
    IconArrowRight,
    AppFooter,
    TechCard,
    IconCommunity,
    ProjectCard,
    Carousel,
    Slide,
    // VueFinalModal,
    IconRight,
    BaseTextInput
  },
  methods: {
    goToContactPage() {
      this.$router.push("/contact");
    },
    /**
     * @function showTechInfo
     * @param techName string - name of the technology e.g rust programming language
     * @param techDescription string - description of the technology
     * @param projects: array of project built with the technonlogy
     */
    showTechInfo(techName: string, techDescription: string, projects: Array<string> = []) {
      console.log("clicked");
      return {
        techName,
        techDescription,
        projects,
      }
    }
  },
  data: () => ({
    breakpoints: {
      // 700px and up
      700: {
        itemsToShow: 2.5,
        snapAlign: "center",
      },
      // 1024 and up
      1024: {
        itemsToShow: 2.75,
        snapAlign: "center",
      },
      hover: false
    },
    showModal: false,
    stack: [
      {
        tech: "rust programming language",
        icon: "rust",
        url: "https://www.rust-lang.org/",
      },

      {
        tech: "Vue.js",
        icon: "vuejs",
        url: "https://www.rust-lang.org/",
      },

      {
        tech: "Linux Operating System",
        icon: "linux",
        url: "https://www.rust-lang.org/",
      },
      {
        tech: "Node.js",
        icon: "nodejs",
        url: "https://www.rust-lang.org/",
      },
      {
        tech: "Typescript",
        icon: "typescript",
        url: "https://www.rust-lang.org/",
      },

      {
        tech: "Syntactically Awesome Stylesheets",
        icon: "postgresql",
        url: "https://www.rust-lang.org/",

      },
    ],
  }),

});
</script>
<template>
  <main>
    <AppNavigation class="view no-mb" />
    <div id="hero" class="view">
      <!--hero text-->
      <div>
        <h1>
          Hey there 👋 <br />
          I&apos;m <span class="emphasis">Opeoluwa</span>,
        </h1>
        <p>
          a software developer driven by need for optimized solution. I take
          interest in scaffolding application&apos;s backend whilst aiming at
          becoming an embedded systems engineer.
        </p>
        <div>
          <router-link :to="{ name: 'projects' }">Explore Projects
            <IconArrowRight />
          </router-link>
          <!-- <a href="#">Download CV</a> -->
        </div>
      </div>
      <!--hero illustration-->
      <div>
        <img src="@/assets/images/ope.png" alt="Opeoluwa" id="avatar" />
      </div>
    </div>

    <!--recent section-->
    <div id="projects" class="view">
      <h2>
        Checkout my <br /><span class="emphasis capitalize">recent Projects</span>
      </h2>
      <Carousel :items-to-show="1.25" :wrap-around="true" :breakpoints="breakpoints">
        <Slide v-for="slide in 10" :key="slide">
          <ProjectCard :title="''" :description="''" />
        </Slide>

        <template #addons> </template>
      </Carousel>
      <router-link :to="{ name: 'projects' }" class="emphasis">view more
        <IconRight />
      </router-link>
    </div>

    <!------------------------------my awesome technology stack---------------------->
    <div id="tech-stack" class="view">
      <h2>
        My
        <span class="emphasis capitalize">core technology</span> <br />
        Stack 🚀
      </h2>
      <!--project card container-->
      <div>
        <div>
          <TechCard v-for="(item, index) in stack" :colored="'colored'" :icon="item.icon" :key="index" />
        </div>
        <div id="tech-info">
          <h3>Technology name</h3>
          <p>
            technology description Lorem ipsum dolor, sit amet consectetur adipisicing elit. Vel nobis doloremque hic
            inventore sint explicabo deserunt
            putting except
          </p>
          <h4>Projects</h4>
          <ul>
            <li>project one</li>
            <li>project one</li>
            <li>project one</li>
            <li>project one</li>
          </ul>
        </div>
      </div>
    </div>

    <!-------------------------------contact me--------------------------->
    <div id="contact-me" class="view">
      <div>
        <h2>
          Send a <br /><span class="emphasis capitalize">Quick</span> message
        </h2>
        <p>Let&apos;s talk about everything</p>

        <BaseTextInput label="Your name" placeholder="Enter your name" />

        <button @click="goToContactPage">
          Send Email
          <IconCommunity />
        </button>
      </div>
      <div>
      </div>
    </div>
  </main>
  <AppFooter />
</template>

<style scoped>
main>div {
  min-height: 75vh;
  padding: 100px unset;
}

main h2 {
  font-weight: 400;
  font-size: 40px;
  line-height: 75px;
  margin-bottom: 90px;
}

#hero {
  display: flex;
  align-items: center;
  justify-content: center;
  background-position: center center;
  background: #ffffff;
  background: -webkit-linear-gradient(bottom, #ffffff, #c9cde0);
  background: -moz-linear-gradient(bottom, #ffffff, #f2f4ff);
  background: linear-gradient(to top, #ffffff, #f2f4ff);
  padding: 70px unset;
}

#hero div:first-child h1 {
  font-family: "Bakbak One";
  font-style: normal;
  font-weight: 400;
  font-size: 64px;
  line-height: 75px;
  margin-bottom: 30px;
  color: #000000;
}

/**sub hero text */
#hero div:first-child h1+p {
  font-style: normal;
  font-weight: 500;
  font-size: 20px;
  line-height: 36px;
  color: #494949;
  object-fit: contain;
  max-width: 80%;
}

#hero div:first-child h1+p::first-letter {
  text-transform: capitalize;
}

/**call to action buttons */
#hero div:first-child div {
  display: flex;
  justify-content: space-between;
  margin-top: 60px;
}

#hero div:first-child div a {
  text-decoration: none;
  font-style: normal;
  font-weight: 500;
  font-size: 18px;
  line-height: 22px;
  background: #000000;
  color: #ffffff;
  padding: 20px 30px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  column-gap: 15px;
}

#hero div:first-child div a svg {
  font-size: 25px;
}

#hero div:first-child div a::last-of-type {
  background: #ffffff;
  color: #000000;
}

/**the hero illustartion */
#avatar {
  width: auto;
  max-width: 500px;
  height: auto;
  object-fit: fill;
}

/*----------------------------about section--------------------- */
#projects {
  text-align: center;
  padding-top: 100px;
  padding-bottom: 100px;
}

#projects h2 {
  font-weight: 400;
  font-size: 40px;
  line-height: 45px;
  margin-bottom: 50px;
}

#projects>a {
  font-weight: 300;
  font-size: 24px;
  line-height: 29px;
  display: inline-flex;
  margin-top: 40px;
  align-items: center;
  text-decoration: none;
  color: var(--default-dark-color);
}

/**---------------------tech stack----------------------- */
#tech-stack {
  background: #ffffff;
  background: -webkit-linear-gradient(bottom, #ffffff, #c9cde0);
  background: -moz-linear-gradient(bottom, #ffffff, #f2f4ff);
  background: linear-gradient(to top, #ffffff, #f2f4ff);
  padding-top: 100px;
  padding-bottom: 100px;
}

#tech-stack h2 {
  line-height: 45px;
}

#tech-stack>div {
  display: flex;
  justify-content: space-between;
  column-gap: 100px;
}

#tech-stack div div:first-child {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  column-gap: 20px;
  row-gap: 35px;
  /* grid-template-rows: ; */
}

/*-------------------------------contact me---------------------------*/
#contact-me {
  padding-top: 50px;
  padding-bottom: 50px;
}

#contact-me h2 {
  font-weight: 400;
  font-size: 40px;
  line-height: 36px;
  margin-bottom: 0px;
  padding: 0 !important;
}

#contact-me h2+p {
  margin-top: 12px;
  margin-bottom: 40px;
  font-size: 24px;
}

#contact-me button {
  font-size: 14px;
  line-height: 20px;
  padding: 15px 20px;
  border-radius: 10px;
  column-gap: 12px;
  background: var(--default-content-color);
  border: 1px solid #efefef;
  display: inline-flex;
  color: var(--white);
  cursor: pointer;
}



/** --------------------------small screens------------------------------- */
@media screen and (max-width: 562px) {


  #hero {
    display: flex;
    align-items: center;
    flex-direction: column;
    padding-top: 50px;
    padding-bottom: 50px;
    margin-bottom: 40px;
    padding-bottom: 0;
  }

  #hero div:first-child h1 {
    font-family: "Bakbak One";
    font-style: normal;
    font-weight: 400;
    font-size: 40px;
    line-height: 40px;
    margin-bottom: 30px;
  }

  /**sub hero text */
  #hero div:first-child h1+p {
    font-style: normal;
    font-weight: 500;
    font-size: 18px;
    line-height: 36px;
    color: #494949;
    object-fit: contain;
    max-width: 85%;
  }

  #hero div:first-child div a {
    font-size: 14px;
    line-height: 20px;
    padding: 15px 20px;
    border-radius: 10px;
    column-gap: 12px;
  }

  /**call to action buttons */
  #hero div:first-child div {
    display: flex;
    justify-content: space-between;
    margin-top: 30px;
  }

  #tech-stack div div:first-child {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    column-gap: 20px;
    row-gap: 25px;
    grid-template-rows: repeat(2, 75px);
    justify-content: center;
  }

  #tech-stack #tech-info {
    display: none;
  }

  #avatar {
    width: 100%;
    max-width: 500px;
    height: auto;
    object-fit: fill;
    margin-top: 100px;
    /* display: none; */
  }
}

@media screen and (max-width: 400px) {
  main>div {
    min-height: 50vh;
    padding: 65px unset;
  }

  main h2 {
    font-weight: 400;
    font-size: 24px;
    line-height: 30px;
  }

  #hero {
    display: flex;
    align-items: center;
    flex-direction: column;
    padding-top: 50px;
    padding-bottom: 50px;
    margin-bottom: 40px;
    padding-bottom: 0;
  }

  #hero div:first-child h1 {
    font-family: "Bakbak One";
    font-style: normal;
    font-weight: 400;
    font-size: 40px;
    line-height: 40px;
    margin-bottom: 30px;
  }

  /**sub hero text */
  #hero div:first-child h1+p {
    font-style: normal;
    font-weight: 500;
    font-size: 16px;
    line-height: 36px;
    color: #494949;
    object-fit: contain;
    max-width: unset;
  }

  /**call to action buttons */
  #hero div:first-child div {
    display: flex;
    justify-content: space-between;
    margin-top: 30px;
  }

  #hero div:first-child div a {
    font-size: 14px;
    line-height: 20px;
    padding: 15px 20px;
    border-radius: 10px;
    column-gap: 12px;
  }

  /*----------------------------about section--------------------- */
  #projects {
    text-align: center;
    padding-top: 40px;
    padding-bottom: 40px;
    margin-bottom: 40px;
  }

  #projects .carousel__prev--in-active,
  #projects .carousel__next--in-active {
    display: none;
  }

  #projects #chevron {
    display: none !important;
  }

  #project>div {
    padding: unset 10px;
  }

  #projects h2 {
    font-weight: 400;
    font-size: 40px;
    font-size: 24px;
    line-height: 30px;
    margin-bottom: 25px;
  }

  #projects>a {
    font-weight: 300;
    font-size: 20px;
    line-height: 20px;
    margin-top: 20px;
  }

  /**---------------------tech stack----------------------- */
  #tech-stack {
    padding-top: 40px;
    padding-bottom: 40px;
    margin-bottom: 40px;
    background: #ffffff;
    background: -webkit-linear-gradient(top, #ffffff, #c9cde0);
    background: -moz-linear-gradient(top, #ffffff, #f2f4ff);
    background: linear-gradient(to bottom, #ffffff, #f2f4ff);
    display: flex;
    flex-direction: column;
    justify-content: center;
    /* align-items: center; */
  }

  #tech-stack h2 {
    font-weight: 400;
    font-size: 40px;
    font-size: 24px;
    line-height: 30px;
    margin-bottom: 25px;
    text-align: center;
  }

  /*-------------------------------contact me---------------------------*/

  #contact-me h2 {
    font-weight: 400;
    font-size: 24px;
    line-height: 26px;
    margin-bottom: 0px;
  }

  #contact-me h2+p {
    margin-top: 12px;
    margin-bottom: 40px;
    font-size: 16px;
  }

}
</style>
