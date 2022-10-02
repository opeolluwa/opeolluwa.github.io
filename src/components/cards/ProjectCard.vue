<template>
  <div class="project carousel__item">
    <h3>{{ title }}</h3>
    <p>{{ description }}</p>
  </div>
</template>

<style scoped>
.project,
.carousel__item {
  min-height: 300px;
  width: 85%;
  font-size: 20px;
  border-radius: 8px;
  border-radius: 10px;
  text-align: left;
  padding: 15px 20px;
  min-height: 300px;
  background-color: #f3efef;
}

.carousel__slide > .carousel__item {
  transform: scale(0.85);
  opacity: 0.5;
  transition: 0.5s;
}

.carousel__slide--visible > .carousel__item {
  opacity: 1;
  transform: rotateY(0);
}

.carousel__slide--next > .carousel__item {
  transform: scale(0.9) translate(-10px);
}

.carousel__slide--prev > .carousel__item {
  transform: scale(0.9) translate(10px);
}

.carousel__slide--active > .carousel__item {
  transform: scale(1.1);
  border-radius: 10px;
}

.carousel__slide--active {
  border-radius: 10px;
}

.project svg {
  border: 1px solid var(--default-dark-color);
  padding: 10px;
  border-radius: 50%;
  object-fit: contain;
  width: 35px;
}

.carousel__slide {
  padding: 10px;
}

.carousel__prev,
.carousel__next {
  box-sizing: content-box;
  border: 5px solid white;
}
</style>

<script lang="ts">
export default {
  name: "ProjectCard",
  props: {
    title: {
      type: String,
      required: true,
    },
    description: {
      type: String,
      required: true,
    },
  },
};
</script>
