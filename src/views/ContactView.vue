<template>
  <AppNavigation class="view" />
  <main class="view">
    <!--contact form modal-->
    <AppModal title="Contact Form" v-show="showModal" @close-modal="showModal = false">
      <template #content>
        <form action="
              ">
          <!--fullname-->
          <BaseTextInput label="Fullname" placeholder="Enter your fullname" />
          <!--email-->
          <BaseTextInput label="Email" placeholder="Enter your email" />
          <!-- subject -->
          <BaseTextInput label="Email subject" placeholder="Enter email subject" />
          <!--message body-->
          <BaseTextarea label="Fullname" placeholder="Enter message body" />
        </form>
      </template>
    </AppModal>
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


