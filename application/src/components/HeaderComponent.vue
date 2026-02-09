<template>
    <div class="centered-top">
        <div class="flex items-center">
            <img src="@/assets/icons/logo.svg" alt="Activity Icon" />
            <h1>Диагностика вирусов у собак</h1>
        </div>
        <p>Клиническая система принятия решений на основе машинного обучения</p>
    </div>
</template>

<style scoped>
.centered-top {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.flex {
    display: flex;
    align-items: center;
}

.items-center {
    align-items: center;
}

.mr-4 {
    margin-right: 1rem;
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
