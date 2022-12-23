<template>
  <!--
    This is a base component for all the text textareas
    It takes in a label and a type
    It also takes in a v-model

    Example usage:
    <BaseTexttextarea label="Name" type="text" v-model="name" />
  -->
  <div class="form__field">
    <label :for="label">{{ label }}</label>
    <textarea :id="label" :placeholder="placeholder" @input="updateModelValue" :value="modelValue" rows="10"
      cols="35"></textarea>
    <!--  <textarea
      :id="label"
      :placeholder="'-- ' + placeholder + ' --'"
      @input="updateModelValue"
      :value="modelValue"
      rows="10"
      cols="35"
    ></textarea> -->
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "BaseTextArea",
  props: {
    label: {
      type: String,
      required: true,
    },
    placeholder: {
      type: String,
      required: true,
    },
    modelValue: {
      type: String,
      required: true,
    },
  },
  methods: {
    updateModelValue(event: any) {
      this.$emit("update:modelValue", event.target.value);
    },
  },
});
</script>

<style scoped>
.form__field {
  margin-bottom: 35px;
  font-size: 16px;
}

.form__field label {
  display: block;
  margin-bottom: 7.5px;
  text-transform: capitalize;
}

.form__field textarea {
  left: 0px;
  max-width: 80%;
  object-fit: contain;
  border-radius: 8px;
  padding: 10px 25px;
  border: 1.5px solid var(--border-color);
  border-radius: 5px;
}

.form__field textarea::placeholder,
::placeholder {
  display: inline-block;
  letter-spacing: 1.25px;
  font-size: 13px;
  font-weight: 400;
  line-height: 21px;
  letter-spacing: 0em;
  text-align: left;
  font-weight: 300;
  font-family: "iter";
}

.form__field textarea:hover,
.form__field textarea:focus {
  border: 1px solid var(--default-dark);
  transition: 0.5s border;
  outline: none;
}

@media screen and (max-width: 768px) {
  .form__field textarea {
    width: 100%;
  }

  .form__field {
    margin-bottom: 35px;
    font-size: 14px;
  }
}
</style>