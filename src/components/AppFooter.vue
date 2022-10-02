<template>
  <footer>
    <div class="view">
      <AppLogo class="d-none" />
      <ul class="d-non">
        <li v-for="(link, index) in links" :key="index">
          <router-link :to="{ name: link.url }">{{ link.path }}</router-link>
        </li>
      </ul>

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

      <!-- <small class="d-none">
        &copy; {{ new Date().getFullYear() }} Opeoluwa - All Rights Reserved
      </small> -->
    </div>
  </footer>
</template>

<style scoped>
footer {
  font-size: small;
  padding-bottom: 20px;
  border-top: 1px solid #e4e4e4;
  padding-top: 25px;
  background-color: #f0eeee;
}

footer > * {
  margin-bottom: 15px;
}

#app-logo {
  justify-content: flex-start;
  align-items: flex-start;
  font-size: 15px;
  margin-bottom: 15px;
}

footer .social {
  display: flex;
  justify-content: flex-start;
  column-gap: 10px;
  align-items: center;
  margin-top: 5px;
  margin-bottom: 20px;
}

footer .social i {
  font-size: 15px;
}

footer ul {
  list-style: none;
  margin: 0;
  padding: 0;
  margin: 18px 0 27px;
}

footer ul li {
  margin-bottom: 5px;
}

footer ul li a,
footer .social a {
  color: #171c17;
  text-decoration: none;
  font-family: "Inter", sans-serif;
  text-transform: capitalize;
  font-weight: 500;
  line-height: 24px;
}

.social {
  margin-top: 0;
}
footer small {
  margin-top: 25px;
  text-align: center;
  display: block;
}
</style>

<script lang="ts">
import AppLogo from "./AppLogo.vue";
export default {
  components: {
    AppLogo,
  },
  data: () => ({
    links: [
      {
        path: "Home",
        url: "home",
      },
      /*   {
        path: "Blog",
        url: "blog",
      }, */
      {
        path: "Projects",
        url: "projects",
      },
      {
        path: "Contact",
        url: "contact",
      },
    ],
  }),
};
</script>
