<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const jsonData = ref({});

async function openFile(fN) {
  await invoke("open_external_app", { fileFullName: fN });
}

async function openCmd(fN) {
  await invoke("run_cmd_file", { fileFullName: fN });
}

async function loadRiConfig() {
  const fileContent = await invoke('read_json_file', { fileName: "D:\\Code\\floating-toolbar\\floating-toolbar\\ri-config.json" });
  jsonData.value = JSON.parse(fileContent);
  console.log(jsonData.value[0].img)
}

loadRiConfig()

</script>

<template>
<div class="d-inline-flex bd-highlight rounded-5 ms-3 py-0 px-2 bg-black bg-opacity-25" data-tauri-drag-region>
            <div class="square-container mx-1 my-1">
                <img class="square-container" src="./img/Steam_icon_logo.svg.png"  
                  v-on:click="openFile('C:\\Program Files (x86)\\Steam\\Steam.exe')"
                  >
            </div>
            <div class="square-container mx-1 my-1">
                <img class="square-container" src="./img/32954.png" alt=""
                  v-on:click="openCmd('D:\\Code\\floating-toolbar\\fff.cmd')">
            </div>
        </div>
</template>

<style scoped>
        .square-container {
            width: 2.5rem;
            height: 2.5rem;
            border-radius: 50%;
            overflow: hidden;
            align-items: center;
            justify-content: center;
            padding: 0%;
            display:inline;
            filter: drop-shadow(0 0 3px #232324);
        }

        .square-container:hover{
          filter: drop-shadow(0 0 5px #000000);
          width: 2.6rem;
          height: 2.6rem;
        }

        .square-container img {
            width: 100%;
            height: 100%;
            object-fit: cover;
            cursor: pointer;
        }
</style>