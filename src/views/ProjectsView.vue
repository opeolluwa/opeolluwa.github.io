<template>
  <AppNavigation class="view" />
  <div class="view container">
    <ul>
      <li v-for="(project, index) in 5" :key="index">
        <ProjectCard />
      </li>
    </ul>
    <IconMore />
  </div>
  <AppFooter />

</template>

<script lang="ts">
import AppNavigation from "@/components/AppNavigation.vue";
import ProjectCard from "@/components/cards/ProjectCard.vue";
import IconMore from "@/components/icons/IconMore.vue";
import AppFooter from "@/components/AppFooter.vue";
export default {
  name: "ProjectsView",
  components: {
    ProjectCard,
    AppNavigation,
    IconMore,
    AppFooter
  },
};
</script>

<style scoped>
.container {
  background-position: center center;
  background: #ffffff;
  background: -webkit-linear-gradient(bottom, #ffffff, #c9cde0);
  background: -moz-linear-gradient(bottom, #ffffff, #f2f4ff);
  background: linear-gradient(to top, #ffffff, #f2f4ff);
}

ul {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  justify-content: center;
  list-style-type: none;
  margin: 0;
  padding: 0;
}

li {
  margin: 35px 0;
  padding: 0;
}

@media screen and (max-width: 768px) {
  ul {
    grid-template-columns: 1fr;
  }
}
</style>
