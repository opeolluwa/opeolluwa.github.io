<template>
  <nav>
    <!--logo-->
    <AppLogo />
    <!--navigation button for small screens-->
    <IconMenu id="nav-btn" />
    <ul>
      <li v-for="(link, index) in links" :key="index">
        <router-link :to="{ name: link.url }">{{ link.path }}</router-link>
      </li>
    </ul>
  </nav>
</template>

<script lang="ts">
import AppLogo from "./AppLogo.vue";
import IconMenu from "./icons/IconMenu.vue";
export default {
  name: "AppNavigation",
  components: {
    AppLogo,
    IconMenu,
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

<style>
/* @media (min-width: 1024px) { */
nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 50px 0;
  margin: 38px 0;
}

#nav-btn {
  display: none;
}

nav ul {
  display: flex;
  justify-content: space-between;
  align-items: center;
  list-style: none;
  padding: 0;
  margin: 0;
  position: relative;
}

nav ul li {
  font-family: "Inter";
  font-style: normal;
  font-weight: 600;
  font-size: 18px;
  line-height: 22px;
  margin-left: 40px;
}

nav ul li a {
  color: #171c17;
  text-decoration: none;
  font-family: "Inter", sans-serif;
  text-transform: capitalize;
  font-weight: 500;
}

nav ul li:last-child {
  box-sizing: border-box;
  display: flex;
  flex-direction: row;
  align-items: flex-start;
  padding: 10px 20px;
  background: var(--default-content-color);
  border-radius: 7.5px;
}

nav ul li:last-child a {
  color: var(--default-white);
}

/*---------------------------smaller screens----------------------- */
@media screen and (max-width: 768px) {
  nav {
    position: relative;
    padding: 30px 0;
    margin: 22px 0;
  }

  #nav-btn {
    display: inline-block;
  }

  nav ul {
    position: absolute;
    left: 0;
    align-items: flex-start;
    display: flex;
    flex-direction: column;
    text-align: left;
    display: none;
  }
}
</style>
