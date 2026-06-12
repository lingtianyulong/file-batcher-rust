<script setup lang="ts">
import TitleBar from "../components/TitleBar.vue";
import MenuBar from "../components/MenuBar.vue";
import SearchBar from "../components/SearchBar.vue";
import MainPage from "./MainPage.vue";
</script>

<template>
  <main class="page">
    <TitleBar title="FileBatcher" :showMinimize="true" :showMaximize="true" />
    <MenuBar />
    <SearchBar />
    <MainPage />
  </main>
</template>

<style scoped>
.page {
  height: 100vh;
  padding: 0;
  background: #ffffff;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}
</style>
