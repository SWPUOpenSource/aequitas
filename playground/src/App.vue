<script setup lang="ts">
import { create_store } from 'aequitas';

const store = create_store("id", {
  name: "John Doe",
  age: 30,
  gender: "Male",
  income: 50000,
})
</script>

<template>
  <main>
    {{ store.get_name() }}
  </main>
</template>

<style scoped>
main {
  background-color: aqua;
}
</style>
