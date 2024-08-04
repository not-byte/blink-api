<script lang="ts" setup>
import { useStorageStore } from "@/stores/storage";
import { useAuthStore } from "@/stores/auth";
import { Ref, ref } from "vue";
import { getTime, trimStr, convert } from "@/utils/util"
import type { LastMessage } from "@declarations/blink_backend.did";
import { storeToRefs } from "pinia";

const last_messages: Ref<LastMessage[]> = ref([]);

const storage = useStorageStore();
const auth = useAuthStore();
storage.$subscribe((_, state) => {
  const { getLastMessages } = storeToRefs(storage);
  last_messages.value = getLastMessages.value;
});
</script>

<template>
  <h2 class="text-[4rem] lg:text-2xl font-semibold">Recent chats</h2>
  <section class="flex flex-col gap-5 rounded-xl overflow-y-scroll no-scrollbar">
    <article class="grid gap-9 lg:gap-3">
      <router-link v-for="message in last_messages" :key="message.timestamp"
        :to="`/messages/${message.conversation_id}`" class="w-full h-fit flex gap-6 lg:gap-3">
        <img v-if="convert(message.conversation_image)" :src="convert(message.conversation_image)" alt=""
          class="h-[9.6rem] lg:h-16 aspect-square rounded-[3rem] lg:rounded-2xl border-2 lg:border border-smoke/10" />
        <img v-else src="https://cdn.yshop.pl/files/RBQ8w.png" alt=""
          class="h-[9.6rem] lg:h-16 aspect-square rounded-[3rem] lg:rounded-2xl border-2 lg:border border-smoke/10" />
        <section class="w-full h-full flex flex-col">
          <h2 class="text-[3rem] lg:text-lg font-semibold">{{ message.user.username }}</h2>
          <p class="text-[2rem] lg:text-base">
            <template v-if="message.user.principal.toText() == auth.identity.getPrincipal().toText()">
              (You)
            </template>
            {{ trimStr(message.content) }}
          </p>
        </section>
        <section class="h-full flex">
          <p class="text-[2rem] lg:text-base">{{ getTime(message.timestamp) }}</p>
        </section>
      </router-link>
    </article>
  </section>
</template>
