<template>
  <v-container>
    <v-row>
      <h1>hello</h1>
      <as-label text="test" widgetId="TextBox1"></as-label>
      <as-button text="test" widgetId="AddButton1" />
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import { initializeView } from '@/plugins/AsFramework';
import { provide } from 'vue';

const { viewmodelId, widgetsData } = initializeView();
provide("viewmodelId", viewmodelId);
provide("widgetsData", widgetsData);
</script>
