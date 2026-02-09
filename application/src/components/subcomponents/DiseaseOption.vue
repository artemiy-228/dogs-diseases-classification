<template>
    <div :class="[{ 'disease-card': !best }, { 'disease-card-best': best }]">
        <div class="top">
            <div class="pos-disease">
                <p class="name">{{ name }}</p>
                <p class="pos" v-if="best">Вероятная болезнь</p>
            </div>

            <div class="percent" :class="levelClass">
                {{ Math.round(possibility * 100) }}%
            </div>
        </div>
        <div class="progress">
            <div
                class="progress-fill"
                :class="levelClass"
                :style="{ width: percentWidth }"
            />
        </div>
    </div>
</template>

<script setup>
import { computed } from "vue";

const props = defineProps({
    name: {
        type: String,
        required: true,
    },
    possibility: {
        type: Number,
        required: true,
    },
    best: {
        type: Boolean,
        default: false,
    },
});

const levelClass = computed(() => {
    if (props.possibility >= 0.8) return "high";
    if (props.possibility >= 0.5) return "medium";
    return "low";
});

const percentWidth = computed(() => `${props.possibility * 100}%`);
</script>

<style scoped>
.disease-card,
.disease-card-best {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    padding: 0.75rem 1rem;
    border: 2px solid #ccc;
    border-radius: 12px;
    background-color: #fff;

    transition: all 0.2s ease-in-out;
}

.disease-card:hover,
.disease-card-best:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
}

.disease-card-best {
    color: #4a5568;
    background: linear-gradient(90deg, #38b2ac 0%, #065f46 100%);
    transition:
        transform 0.2s ease,
        box-shadow 0.2s ease;
}

.disease-card-best .name {
    color: #fff;
}

.top {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.name {
    font-size: 0.9rem;
    color: #4a5568;
    font-weight: 500;
}

.percent {
    min-width: 64px;
    text-align: center;
    padding: 0.25rem 0.75rem;
    border-radius: 999px;
    font-weight: 600;
    font-size: 0.85rem;
}

.progress {
    width: 100%;
    height: 6px;
    background-color: #e5e7eb;
    border-radius: 999px;
    overflow: hidden;
}

.progress-fill {
    height: 100%;
    border-radius: 999px;
    transition: width 0.4s ease;
}

.progress-fill.high,
.percent.high {
    background-color: #34d399;
    color: #065f46;
}

.progress-fill.medium,
.percent.medium {
    background-color: #fbbf24;
    color: #92400e;
}

.progress-fill.low,
.percent.low {
    background-color: #f87171;
    color: #991b1b;
}

.pos-disease {
    display: flex;
    align-items: center;
    gap: 1rem;
}

.pos-disease .pos {
    font-size: 10px;
    background-color: #38b2ac;
    border-radius: 999px;
    padding: 0.1rem 0.25rem;
    color: #fff;
}
</style>
