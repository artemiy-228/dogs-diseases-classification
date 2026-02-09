<template>
    <label :for="id" :class="['symptom-card', checked ? 'checked' : '']">
        <input
            type="checkbox"
            :id="id"
            :checked="checked"
            @change="onCheckedChange($event.target.checked)"
        />
        <span>{{ label }}</span>
    </label>
</template>

<script setup>
const props = defineProps({
    id: String,
    label: String,
    checked: Boolean,
});

const emit = defineEmits(["update:checked"]);

function onCheckedChange(value) {
    emit("update:checked", value ? 1 : 0);
}
</script>

<style scoped>
.symptom-card {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    border: 2px solid #ccc;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s ease-in-out;
    background-color: #fff;
}

.symptom-card.checked {
    background-color: #d1fae5;
    border-color: #38b2ac;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.symptom-card input {
    width: 1rem;
    height: 1rem;
}

.symptom-card span {
    font-size: 0.875rem;
    color: #4a5568;
}

.symptom-card.checked span {
    font-weight: 500;
    color: #065f46;
}
</style>
