<template>
  <div id="app-logo">
    <router-link :to="{ name: 'home' }"
      >Opeolluwa<span class="emphasis">.</span>
    </router-link>
  </div>
</template>

<script lang="ts">
export default {
  name: "AppLogo",
};
</script>

<style scoped>
#app-logo {
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 25px;
  font-weight: bold;
}
#app-logo a {
  color: var(--default-dark-color);
  font-family: "Bakbak One", cursive;
  text-decoration: none;
}
</style>
