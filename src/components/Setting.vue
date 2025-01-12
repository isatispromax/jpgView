<!-- src/components/Home.vue -->
<template>
    <div class="container text-center">
        <div class="row">
            <div class="col">
                <div class="input-group">
                    <input type="file" class="form-control" id="inputGroupFile04" webkitdirectory directory
                        aria-describedby="inputGroupFileAddon04" aria-label="Upload">
                    <button class="btn btn-outline-secondary" type="button" id="inputGroupFileAddon04">Button</button>
                </div>
            </div>
        </div>
        <div class="row">
            <div class="col">
                1 of 3
            </div>
            <div class="col">
                2 of 3
            </div>
            <div class="col">
                3 of 3
            </div>
        </div>
    </div>
</template>

<script>
// import * from 'bootstrap'
import { ref, onMounted } from "vue";
import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap/dist/js/bootstrap'
import { appDataDir } from '@tauri-apps/api/path';
import { writeFile, readFile } from '@tauri-apps/plugin-fs'
import { invoke } from "@tauri-apps/api/core";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";

const CONFIG_FILE_NAME = 'config.json';

const defaultConfig = {
    imageDirectory: '',
    switchImageShortcut: ''
};

export default {
    name: 'Setting',
    setup() {
        const getConfig = async () => {
            try {
                const appData = await appDataDir();
                const configPath = `${appData}/${CONFIG_FILE_NAME}`;
                console.log("config path:" + configPath);
                const configData = await invoke("read_config_file_context", { filePath: configPath });
                console.log("config data: " + configData);
                return JSON.parse(configData);
            } catch (error) {
                console.error('读取配置文件失败:', error);
                await invoke("save_config_file", { filePath: configPath, content: JSON.stringify(defaultConfig, null, 2) });
                return defaultConfig;
            }
        };

        const saveConfig = async (config) => {
            try {
                const appData = await appDataDir();
                const configPath = `${appData}/${CONFIG_FILE_NAME}`;
                const configData = JSON.stringify(config, null, 2);
                await writeFile({ path: configPath, contents: configData });
                console.log('配置文件保存成功');
            } catch (error) {
                console.error('保存配置文件失败:', error);
            }
        };

        onMounted(() => {
            console.log("setting load");
            getConfig();
        });
    }
}
</script>

<style scoped>
/* Add any styles you want here */
</style>