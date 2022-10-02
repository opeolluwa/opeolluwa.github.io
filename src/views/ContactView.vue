<template>
  <AppNavigation class="view" />
  <main class="view">
    Lorem ipsum dolor sit, amet consectetur adipisicing elit. Atque corrupti ut a distinctio nostrum molestiae id q
    porro reiciendis. Enii impedit ullam fugit fugiat libero corporis voluptate officiis perspiciatis!
  </main>
  <AppFooter />
</template>

<script lang="ts">
import AppNavigation from "@/components/AppNavigation.vue";
import IconMore from "@/components/icons/IconMore.vue";
import AppFooter from "@/components/AppFooter.vue";
export default {
  name: "ProjectsView",
  components: {
    AppNavigation,
    IconMore,
    AppFooter
  },
};
</script>


