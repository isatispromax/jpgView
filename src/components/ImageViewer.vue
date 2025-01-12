<template>
    <div class="image-viewer">
        <img :src="currentImage" alt="Image" class="full-screen-image" v-if="currentImage" />
        <p v-else>No images found</p>
    </div>
</template>

<script>
import { ref, onMounted } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { appDataDir } from '@tauri-apps/api/path';
import { readDir, BaseDirectory, exists, mkdir, copyFile } from '@tauri-apps/plugin-fs';
import { register } from '@tauri-apps/plugin-global-shortcut';

export default {
    setup() {
        const images = ref([]); // 存储图片路径
        const currentIndex = ref(0); // 当前图片索引
        const currentImage = ref(null); // 当前显示的图片路径

        // 获取图片列表
        const fetchImages = async () => {
            const appDataDirPath = await appDataDir();
            console.log("appData:" + appDataDirPath);
            const appTmp = "/Users/yangdi/code/photoSwitch/jpgNpmView/jpgView";

            console.log(appTmp);
            //const files = await readDir(appTmp); // 你可以根据需要修改目录

            try {
                const imagePaths = await invoke("get_images_from_directory", { imgDir: appTmp });
                console.log(imagePaths);
                images.value = imagePaths;
                if (images.value.length > 0) {
                    const imgCur = convertFileSrc(images.value[currentIndex.value]);
                    console.log("imgCur：  " + imgCur);

                    currentImage.value = imgCur;
                }
            } catch (error) {
                console.error("Error fetching images:", error);
            }
        };

        // 切换图片
        const changeImage = (direction) => {
            if (images.value.length === 0) return;
            if (direction === "next") {
                currentIndex.value = (currentIndex.value + 1) % images.value.length;
            } else if (direction === "prev") {
                currentIndex.value = (currentIndex.value - 1 + images.value.length) % images.value.length;
            }
            currentImage.value = convertFileSrc(images.value[currentIndex.value]);
            console.log(currentImage);
        };

        // 监听键盘事件
        const handleKeyDown = (event) => {
            console.log("event:::::" + event);
            if (event.key === "ArrowRight" || event.key === "ArrowDown") {
                changeImage("next");
            } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
                changeImage("prev");
            }
        };

        // 在组件挂载时获取图片并设置键盘事件监听
        onMounted(() => {
            fetchImages();
            window.addEventListener("keydown", handleKeyDown);
        });

        return { currentImage };
    }
};
</script>

<style scoped>
.image-viewer {
    position: absolute;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    margin: 0;
    padding: 0;
    overflow: hidden;
}


.full-screen-image {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
}
</style>