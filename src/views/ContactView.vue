<template>
  <AppNavigation class="view" />
  <div class="view container">
    <!--contact form modal-->
    <h2>
      Contact Me<br />
      <small class="text-primary emphasis">Le&apos;s talk about </small> <small>everything 🤗</small>
    </h2>
    <p>
      Please fill out the form below and I will get back to you
      as soon as possible.
    </p>
    <form @submit.prevent="sendEmail">
      <!--fullname-->
      <BaseTextInput label="Fullname" placeholder="Enter your fullname" v-model="senderName" />
      <!--email-->
      <BaseTextInput label="Email" placeholder="Enter your email" v-model="senderEmail" />
      <!-- subject -->
      <BaseTextInput label="Email subject" placeholder="Enter email subject" v-model="emailSubject" />
      <!--message body-->
      <BaseTextarea label="Message" placeholder="Enter message body" v-model="emailBody" />
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
import { defineComponent } from "vue";
export default defineComponent({
  name: "ProjectsView",
  components: {
    AppNavigation,
    IconMore,
    AppFooter,
    BaseTextInput,
    BaseTextarea,
    BaseButton,
  },
  data: () => ({
    senderName: "",
    senderEmail: "",
    emailSubject: "",
    emailBody: ""
  }),
  methods: {
    sendEmail() {
      console.log("sending email", {
        senderName: this.senderName,
        senderEmail: this.senderEmail,
        emailSubject: this.emailSubject,
        emailBody: this.emailBody
      });
    },
  },
});
</script>


<style  scoped>
/* nav {
  padding: 20px unset;
  background-color: var(--primary);
  min-height: 70px;
  color: var(--white);
  margin-top: 0;
} */

.container {
  padding-top: 20px;
  padding-bottom: 100px;
  background-position: center center;
  background: #ffffff;
  background: -webkit-linear-gradient(bottom, #ffffff, #c9cde0);
  background: -moz-linear-gradient(bottom, #ffffff, #f2f4ff);
  background: linear-gradient(to top, #ffffff, #f2f4ff);
}

h2 {
  font-weight: 400;
  font-size: 24px;
  margin-bottom: 5px;
}

p {
  margin-top: 20px;
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

form button {
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