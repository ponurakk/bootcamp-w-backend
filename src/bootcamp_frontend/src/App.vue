<script setup lang="ts">
import { Ref, ref } from "vue";
import { bootcamp_backend } from "declarations/bootcamp_backend/index";
import Blog from "@/components/Blog.vue";
let greeting = ref('');

let name: Ref<string> = ref('');
let age: Ref<number> = ref(0);

async function handleSubmit(e: any) {
  e.preventDefault();
  let response = await bootcamp_backend.greet(name, age);
  greeting.value = response;
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <span class="block">
        <label for="name">Enter your name: </label>
        <input id="name" alt="Name" type="text" v-model="name" class="p-1" />
      </span>
      <span class="block">
        <label for="age">Enter your age: </label>
        <input id="age" alt="Age" type="number" v-model="age" class="p-1"/>
      </span>
      <button type="submit" class="bg-red-900">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>
  </main>

  <Blog/>
</template>
