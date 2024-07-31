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
  <section class="w-full flex flex-col gap-3">
    <h2 class="text-2xl font-semibold">Recent chats</h2>
    <article class="overflow-visible">
      <router-link :to="`/${message.conversation_id}`" v-for="message in last_messages" :key="message.timestamp">
        <aside class="w-full h-fit flex justify-between">
          <img v-if="message.user.avatar" :src="message.user.avatar" :alt="message.user.username"
            class="h-[64px] w-[64px]" />
          <img v-else src="https://friconix.com/png/fi-cnsuxl-user-circle.png" alt="" class="h-[64px] w-[64px]" />
          <div class="p-2 px-4">
            <h2 class="font-semibold text-xl">{{ message.user.username }}</h2>
            <p>
              <template v-if="message.user.caller == auth.identity.getPrincipal()">
                (You)
              </template>
              {{ trimStr(message.content) }}
            </p>
          </div>
          <div class="py-2">
            <p>{{ getTime(message.timestamp) }}</p>
          </div>
        </aside>
      </router-link>
    </article>
  </section>
</template>
