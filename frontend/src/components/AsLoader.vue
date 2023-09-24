<template>
    <div>
        <slot v-if="widgetsData._loader === 'Done'">

        </slot>
        <div v-else-if="widgetsData._loader === 'Loading'">
            Loading...
        </div>
        <div v-else>
            Error: {{ widgetsData._loader.message }}
        </div>
    </div>
</template>

<script lang="ts" setup>
import { inject } from 'vue';

const widgetsData: any = inject("widgetsData");
</script>
