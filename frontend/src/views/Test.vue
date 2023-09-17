<template>
    <h1>hello</h1>
</template>

<script lang="ts" setup>
import { initializeView } from '@/plugins/AsFramework';
import { provide } from 'vue';

const { viewmodelId, widgetsData } = initializeView();
provide("viewmodelId", viewmodelId);
provide("widgetsData", widgetsData);
</script>
