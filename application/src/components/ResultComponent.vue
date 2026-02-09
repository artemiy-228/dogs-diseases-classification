<template>
    <div class="diagnosis-placeholder">
        <h3>Результаты диагностики</h3>

        <div class="empty" v-if="diseases.length === 0">
            <div class="icon">
                <img src="@/assets/icons/logo.svg" alt="Activity Icon" />
            </div>
            <p class="title">Пока ничего нет</p>
            <p class="subtitle">
                Выберите симптомы из левой панели и нажмите "Диагностика", чтобы
                увидеть прогнозы вирусных заболеваний
            </p>
        </div>
        <div class="place" v-else>
            <DiseaseOption
                v-for="(disease, index) in diseases"
                :key="disease.name"
                :name="disease.name"
                :possibility="disease.possibility"
                :best="index === 0"
            />
        </div>

        <div class="space"></div>
    </div>
</template>

<script setup>
import DiseaseOption from "./subcomponents/DiseaseOption.vue";

const props = defineProps({
    diseases: {
        type: Array,
        default: () => [],
    },
});
</script>

<style scoped>
h3 {
    margin: 0;
    padding: 0;
    font-size: 18px;
    color: #111827;
    margin-bottom: 1rem;
}

.diagnosis-placeholder {
    display: flex;
    flex-direction: column;
}

.diagnosis-placeholder > .empty {
    flex: 8;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.diagnosis-placeholder > .place {
    flex: 8;
    display: flex;
    flex-direction: column;
}

.diagnosis-placeholder .icon {
    width: 64px;
    height: 64px;
    border-radius: 50%;
    background-color: #d9f9f9;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 12px;
}

.diagnosis-placeholder .title {
    font-weight: 600;
    font-size: 16px;
    color: #111827;
    margin-bottom: 4px;
    text-align: center;
}

.diagnosis-placeholder .subtitle {
    font-size: 14px;
    color: #6b7280;
    max-width: 220px;
    text-align: center;
    line-height: 1.4;
}

.place {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 0.5rem;

    max-height: 100%;
    overflow-y: auto;
    padding-right: 0.5rem;
    margin-top: 1.5rem;
}

.space {
    flex: 1;
}
</style>
