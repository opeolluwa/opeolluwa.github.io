<template>
  <AppNavigation class="view" />
  <div class="view container">
    <!--contact form modal-->
    <h2>
      <span class="text-primary">Contact</span> Me
    </h2>
    <p>
      If you have any questions or would like to get in touch, please fill out the form below and I will get back to you
      as soon as possible.
    </p>
    <form action="">
      <!--fullname-->
      <BaseTextInput label="Fullname" placeholder="Enter your fullname" />
      <!--email-->
      <BaseTextInput label="Email" placeholder="Enter your email" />
      <!-- subject -->
      <BaseTextInput label="Email subject" placeholder="Enter email subject" />
      <!--message body-->
      <BaseTextarea label="Message" placeholder="Enter message body" />
      <!--submit button-->
      <BaseButton text="Send Message" />
    </form>

  </div>
  <AppFooter />
</template>

<script lang="ts">
import AppNavigation from "@/components/AppNavigation.vue";
import IconMore from "@/components/icons/IconMore.vue";
import AppFooter from "@/components/AppFooter.vue";
import BaseTextInput from "@/components/BaseTextInput.vue";
import BaseTextarea from "@/components/BaseTextarea.vue";
import BaseButton from "@/components/BaseButton.vue";
export default {
  name: "ProjectsView",
  components: {
    AppNavigation,
    IconMore,
    AppFooter,
    BaseTextInput,
    BaseTextarea,
    BaseButton,
  },
};
</script>


<style  scoped>
nav {
  padding: 20px unset;
  background-color: var(--primary);
  min-height: 70px;
  color: var(--white);
  margin-top: 0;
}

.container {
  padding-top: 20px;
  padding-bottom: 100px;
}

h2 {
  font-weight: 400;
  font-size: 24px;
  margin-bottom: 5px;
}

p {
  margin-top: 0;
  font-size: 16px;
  line-height: 30px;
  margin-bottom: 60px;
}

form {
  display: grid;
  grid-template-columns: 1fr;
  grid-gap: 1rem;
}

form .form-field input {
  border-radius: 5px;
}

form button{
  width: 100%;
  height: 50px;
  border-radius: 5px;
  background-color: var(--primary);
  color: var(--white);
  font-size: 16px;
  font-weight: 600;
  border: none;
  cursor: pointer;
}
</style>