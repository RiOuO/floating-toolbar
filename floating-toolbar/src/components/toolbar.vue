<script setup>
import { ref, onMounted } from "vue";
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
}

async function resizeWindow() {
    const width = document.body.scrollWidth;
    const height = document.body.scrollHeight;
    await invoke("resize_window", { width, height });
}

const getImageSrc = (imageName) => {
  return new URL(`./img/${imageName}`, import.meta.url).href;
};

onMounted(async () => {
  await loadRiConfig();
  await resizeWindow();
});

</script>

<template>
  <div class="d-inline-flex bd-highlight rounded-5 ms-3 py-0 px-2 bg-black bg-opacity-25" data-tauri-drag-region>
    <div class="square-container mx-1 my-1" v-for="(item, index) in jsonData">
      <img class="square-container" :src="getImageSrc(item.img)"
        v-on:click="item.fn.value == 'exe'? () => openFile(item.path) : openCmd(item.path)" data-toggle="tooltip" :title="item.mes">
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
  display: inline;
  filter: drop-shadow(0 0 3px #232324);
}

.square-container:hover {
  filter: drop-shadow(0 0 5px #8d2828);
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