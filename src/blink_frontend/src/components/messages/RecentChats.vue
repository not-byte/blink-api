<script lang="ts" setup>
import { useStorageStore } from "@/stores/storage";
import { useAuthStore } from "@/stores/auth";
import { ref } from "vue";
import { getTime, trimStr } from "@/utils/util"
import type { LastMessage } from "../../../../../declarations/blink_backend/blink_backend.did";

const last_messages = ref<LastMessage[]>([])

const storage = useStorageStore();
const auth = useAuthStore();
setTimeout(() => {
  last_messages.value = storage.getLastMessages;
}, 2000)
</script>

<template>
  <h2 class="text-2xl font-semibold">Recent chats</h2>
  <section class="flex flex-col gap-5 rounded-xl overflow-y-scroll no-scrollbar">
    <article class="grid gap-3">
      <router-link v-for="message in last_messages" :key="message.timestamp"
        :to="`/messages/${message.conversation_id}`" class="w-full h-fit flex gap-3">
        <img v-if="message.user.avatar" :src="message.user.avatar" alt="" class="h-14 aspect-square" />
        <img v-else src="https://friconix.com/png/fi-cnsuxl-user-circle.png" alt="" class="h-14 aspect-square" />
        <section class="w-full h-full flex flex-col">
          <h2 class="font-semibold text-lg">{{ message.user.username }}</h2>
          <p>
            <template v-if="message.user.caller == auth.identity.getPrincipal()">
              (You)
            </template>
            {{ trimStr(message.content) }}
          </p>
        </section>
        <section class="h-full flex">
          <p>{{ getTime(message.timestamp) }}</p>
        </section>
      </router-link>
    </article>
  </section>
</template>
