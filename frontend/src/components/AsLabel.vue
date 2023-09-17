<template>
  <p>{{ data.text }}</p>
</template>

<script lang="ts" setup>
import { initializeWidget } from '@/plugins/AsFramework';

const props = defineProps(["widgetId", "text"]);
const { viewmodelId, data } = initializeWidget(props);
</script>
