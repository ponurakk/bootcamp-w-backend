<script setup lang="ts">
import { bootcamp_backend } from "declarations/bootcamp_backend/index";
import { onMounted, Ref, ref } from "vue";

let new_post: Ref<string> = ref('');
let posts: Ref<string[]> = ref([]);

async function addPost() {
  console.time("AddPost");
  await bootcamp_backend.add_post(new_post.value)
  console.timeEnd("AddPost");
  console.time("GetPosts");
  posts.value = await bootcamp_backend.get_posts();
  console.timeEnd("GetPosts");
}

async function removePost(index: number) {
  console.time("RemovePost");
  await bootcamp_backend.remove_post(index)
  console.timeEnd("RemovePost");
  posts.value = await bootcamp_backend.get_posts();
}

async function clear() {
  console.time("Clear");
  await bootcamp_backend.clear()
  console.timeEnd("Clear");
  posts.value = [];
}
  
onMounted(async () => {
  posts.value = await bootcamp_backend.get_posts();
})
</script>

<template>
  <div>
    <section class="flex gap-3">
      <input type="text" v-model="new_post" class="px-3 py-1">
      <button @click="addPost" class="bg-indigo-700 rounded-sm p-1">Add Post</button>
      <button @click="clear" class="bg-indigo-700 rounded-sm p-1">Clear</button>
    </section>

    <section>
      <h1>POSTS:</h1>
      <div class="flex flex-col gap-3 w-1/2 m-auto">
        <div v-for="(post, i) in posts" :key="i" class="bg-neutral-50 rounded-xl p-4">
          <div>{{ post }}</div>
          <button @click="removePost(i)" class="bg-red-900 rounded-sm p-1">Remove</button>
        </div>
      </div>
    </section>
  </div>
</template>
