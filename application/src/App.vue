<template>
    <div class="page">
        <div class="header">
            <HeaderComponent />
        </div>
        <div class="body">
            <SymptompsComponent @disease="handleDisease" @reset="handleReset" />
            <ResultComponent :diseases="diseases" />
        </div>
        <div class="footer">
            <FooterComponent />
        </div>
    </div>
</template>

<script setup>
import HeaderComponent from "./components/HeaderComponent.vue";
import FooterComponent from "./components/FooterComponent.vue";
import SymptompsComponent from "./components/SymptompsComponent.vue";
import ResultComponent from "./components/ResultComponent.vue";

import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

let diseases = ref([]);

function handleReset() {
    diseases.value = [];
}

async function handleDisease(result) {
    try {
        const rawData = await invoke("predict_diseases", { data: result });

        console.log(rawData);

        const sortedArray = Object.entries(rawData)
            .map(([name, possibility]) => ({ name, possibility }))
            .sort((a, b) => b.possibility - a.possibility);

        diseases.value = sortedArray;
    } catch (e) {
        console.error(e);
        alert(String(e));
    }
}
</script>

<style scoped>
.page {
    display: flex;
    flex-direction: column;
    height: 85vh;
    background-color: #f5f5f5;
    padding: 3rem 8rem;
}

.body {
    display: flex;
    justify-content: center;
    flex-wrap: wrap;
    gap: 2rem;
}

.body > * {
    flex: 1 1 auto;
    min-width: 400px;
    max-width: 500px;
    height: 700px;
}

h1 {
    margin: 0;
}

p {
    margin-top: 0.5rem;
    font-size: 0.8rem;
    color: #4a5565;
}

img {
    height: 48px;
}
</style>
