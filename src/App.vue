<script>
import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap/dist/js/bootstrap'
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ImageViewer from "./components/ImageViewer.vue";
import SlideMenu from "./components/SlideMenu.vue";

const greetMsg = ref("");
const name = ref("");

export default {
  components: {
    SlideMenu
  }
}
</script>

<template>
  <div><router-view /></div>
  <div>
    <SlideMenu />
  </div>
</template>

<style>
html,
body {
  margin: 0;
  padding: 0;
}
</style>
