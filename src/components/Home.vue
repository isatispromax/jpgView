<!-- src/components/Home.vue -->
<template>
    <div id="homeView">
        <ImageViewer />
    </div>
</template>

<script>
import ImageViewer from './ImageViewer.vue';

export default {
    name: 'Home',
    components: {
        ImageViewer
    }
}
</script>

<style scoped>
/* Add any styles you want here */
#homeView {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: #2c3e50;
    margin-top: 60px;
}
</style>