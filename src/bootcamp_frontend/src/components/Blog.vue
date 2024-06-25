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
  
onMounted(async () => {
  posts.value = await bootcamp_backend.get_posts();
})
</script>

<template>
  <div>
    <section>
      <input type="text" v-model="new_post">
      <button @click="addPost">Add Post</button>
    </section>

    <section>
      <h1>POSTS:</h1>
      <div v-for="(post, i) in posts" :key="i">
        {{ post }} <button @click="removePost(i)">Remove</button>
      </div>
    </section>
  </div>
</template>
