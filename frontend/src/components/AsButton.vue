<template>
  <v-btn @click="click">{{ data.text }}</v-btn>
</template>

<script lang="ts" setup>
import { initializeWidget } from '@/plugins/AsFramework';

const props = defineProps(["widgetId", "text"]);
const { viewmodelId, data, postAction } = initializeWidget(props);
const click = () => {
  postAction("click");
}
</script>
