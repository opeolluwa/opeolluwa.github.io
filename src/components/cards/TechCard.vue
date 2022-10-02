<template>
  <div :class="['devicon-' + icon + '-plain', colored]"></div>
</template>

<script lang="ts">
export default {
  name: "TechCard",
  props: {
    icon: {
      type: String,
      required: true,
    },
    colored: {
      type: String,
    },
  },
};
</script>

<style scoped>
@media screen and (min-width: 768px) {
  div {
    font-size: 100px;
    display: inline-block;
    padding: 5px;
    border: 1px solid #f2f2f2;
    border-radius: 5px;
    filter: grayscale(100%) brightness(45%);
    -webkit-filter: grayscale(100%) brightness(45%);
    -moz-filter: grayscale(100%) brightness(45%);
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
  }
}

@media screen and (min-width: 562px) {
  div {
    font-size: 65px;
    display: inline-block;
    padding: 5px;
    border: 1px solid #f2f2f2;
    border-radius: 5px;
    filter: grayscale(100%) brightness(45%);
    -webkit-filter: grayscale(100%) brightness(45%);
    -moz-filter: grayscale(100%) brightness(45%);
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
  }
}
div {
  font-size: 50px;
  display: inline-flex;
  padding: 5px;
  border-radius: 5px;
  filter: grayscale(100%) brightness(45%);
  -webkit-filter: grayscale(100%) brightness(45%);
  -moz-filter: grayscale(100%) brightness(45%);
  cursor: pointer;
}
</style>
