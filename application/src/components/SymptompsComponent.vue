<template>
    <div class="symptomps">
        <h3>Выберите симптомы</h3>
        <p>{{ selectedCount }} выбрано</p>

        <div class="options">
            <SymptomOption
                v-for="(symptom, index) in SYMPTOMS"
                :key="symptom"
                :id="symptom"
                :label="symptom"
                v-model:checked="selected[index]"
            />
        </div>
        <div class="buttons">
            <button
                class="predict-button"
                :disabled="selectedCount === 0"
                @click="$emit('disease', getSelected())"
            >
                Диагностика
            </button>
            <button
                class="reset-button"
                :disabled="selectedCount === 0"
                @click="$emit('reset', reset())"
            >
                Сбросить
            </button>
        </div>
    </div>
</template>

<script setup>
import { reactive, computed } from "vue";
import { defineProps } from "vue";
import SymptomOption from "./subcomponents/SymptomOption.vue";

const SYMPTOMS = [
    "Высокая температура",
    "Выраженная депрессия",
    "Вялость",
    "Мышечная дрожь",
    "Галлюцинации",
    "Пугливость",
    "Агрессивность",
    "Стремление бежать",
    "Поражение ЦНС",
    "Судороги",
    "Эпилептические припадки",
    "Паралич мышц",
    "Нарушение координации",
    "Серозно-слизистые, а затем гнойные истечения из носа",
    "Серозно-слизистые, а затем гнойные истечения из глаз",
    "Конъюнктивит",
    "Помутнение зрения",
    "Пустулезная сыпь",
    "Приступы рвоты",
    "Затруднение глотания",
    "Извращенный аппетит",
    "Тошнота, рвота с примесью крови",
    "Обезвоживание",
    "Проблемы с ЖКТ",
    "Отказ от пищи",
    "Обильное слюнотечение",
    "Поражение респираторной системы",
    "Анорексия",
    "Постоянный плач",
    "Сильный зуд и расчёсы",
];

const selected = reactive(Array(SYMPTOMS.length).fill(0));

const selectedCount = computed(() => selected.filter((v) => v === 1).length);

function reset() {
    selected.fill(0);
}

const props = defineProps({
    diseases: Object,
});

const emits = defineEmits(["disease"]);
const getSelected = () => [...selected];
</script>

<style scoped>
.symptomps {
    display: flex;
    flex-direction: column;
}

.scroll-menu {
    flex: 9;
}

h3,
p {
    padding: 0;
    margin: 0;
}

p {
    padding-top: 0.5rem;
    padding-left: 1rem;
    color: #4a5565;
    font-size: 14px;
}

.options {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 0.5rem;
    margin-top: 1rem;

    max-height: 100%;
    overflow-y: auto;
    padding-right: 0.5rem;
}

.buttons {
    display: flex;
}

.reset-button {
    margin-top: 1rem;
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    font-weight: 500;
    border: none;
    border-radius: 12px;
    cursor: pointer;

    display: inline-block;
    flex: 1;
}

.reset-button:disabled {
    cursor: not-allowed;
    opacity: 0.5;
    display: none;
}

.reset-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.reset-button:active {
    transform: translateY(0);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
}

.predict-button {
    margin-top: 1rem;
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    font-weight: 500;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    color: #fff;

    background: linear-gradient(90deg, #38b2ac 0%, #065f46 100%);
    transition:
        transform 0.2s ease,
        box-shadow 0.2s ease;
    flex: 5;
}

.predict-button:disabled {
    cursor: not-allowed;
    opacity: 0.5;
    background: linear-gradient(90deg, #38b2ac 0%, #065f46 100%);
}

.predict-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.predict-button:active {
    transform: translateY(0);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
}
</style>
