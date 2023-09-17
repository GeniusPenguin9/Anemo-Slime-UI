<template>
  <v-app id="inspire">
    <v-navigation-drawer v-model="drawer">
      <v-list lines="two">
        <v-list-item v-for="viewName in viewNames" :key="viewName">
          <router-link :to="viewName">{{ viewName }}</router-link>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-app-bar>
      <v-app-bar-nav-icon @click="drawer = !drawer"></v-app-bar-nav-icon>

      <v-app-bar-title><router-link to="/view/example">Application</router-link></v-app-bar-title>
    </v-app-bar>

    <v-main>
      <router-view :key="$route.fullPath">
      </router-view>
    </v-main>
  </v-app>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { views } from "@/views/index"
import { computed } from 'vue';

const drawer = ref(true);
const viewNames = computed(() => {
  return Object.keys(views);
});
</script>
