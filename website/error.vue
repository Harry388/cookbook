<script setup lang="ts">
    const props = defineProps({
        error: Object
    });
</script>

<template>
    <div class="hero min-h-screen bg-base-200">
        <div class="hero-content text-center">
            <div class="max-w-md">
                <h1 class="text-5xl font-bold">Error {{ props.error!.statusCode }}</h1>
                <p class="py-6">{{ props.error!.message }}</p>
                <NuxtLink class="btn btn-primary" to="/">Go Home</NuxtLink>
            </div>
        </div>
    </div>
</template>
