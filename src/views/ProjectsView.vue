<template>
  <AppNavigation class="view" />
  <main class="view">
    <ul>
      <li v-for="(project, index) in 5" :key="index">
        <ProjectCard />
      </li>
    </ul>
    <IconMore />
  </main>
  <AppFooter  />

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
