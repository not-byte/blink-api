<script lang="ts" setup>
import { useStorageStore } from "@/stores/storage";

setTimeout(() => {
  const storage = useStorageStore();
  console.log("storage:", storage.getConversations);
}, 3000)
</script>

<template>
  <section class="w-full flex flex-col gap-3">
    <h2 class="text-2xl font-semibold">Recent chats</h2>
    <article class="overflow-visible">
      <aside v-for="user in users" :key="user.fullname" class="w-full h-fit flex justify-between">
        <img :src="user.avatar" alt="" class="h-[64px] w-[64px]" />
        <div class="p-2 px-4">
          <h2 class="font-semibold text-xl">{{ user.fullname }}</h2>
          <p>{{ user.message }}</p>
        </div>
        <div class="py-2">
          <p>{{ user.sent }}</p>
        </div>
      </aside>
    </article>
  </section>
</template>
