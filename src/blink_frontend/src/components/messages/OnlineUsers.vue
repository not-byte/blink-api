<script lang="ts" setup>
import { useAuthStore } from "@/stores/auth";
import { useStorageStore } from "@/stores/storage";
import { convert, unwrap, extractId } from "@/utils/util";
import type { User, Conversation } from "@declarations/blink_backend.did";
import { Principal } from "@dfinity/principal";
import { ref, Ref } from "vue";
import { useRouter } from "vue-router";
const auth = useAuthStore();
const storage = useStorageStore();

const users: Ref<User[]> = ref([]);
const router = useRouter();

async function createConversation(user: Principal) {
  let conversation_id = 0;
  try {
    conversation_id = Number(unwrap(await auth?.actor.create_conversation([user])));
    const conversations: Conversation[] = unwrap(await auth.actor?.get_user_conversations());
    storage.setConversations(conversations);
    await router.push(`/messages/${conversation_id}`);
  } catch (e) {
    console.error(e);
    if (e.message.startsWith("Conversation already exists")) {
      let id = extractId(e.message);
      await router.push(`/messages/${id}`);
    }
  }
}

(async () => {
  let new_users = await auth?.actor.get_users();
  users.value = new_users.filter(v => v.principal !== auth.getPrincipal);
})()
</script>

<template>
  <section class="flex flex-col gap-3">
    <!-- <h2 class="text-2xl font-semibold">Online</h2> -->
    <h2 class="text-[4rem] lg:text-2xl font-semibold">List of users →</h2>
    <aside class="flex gap-8 lg:gap-[0.85rem] rounded-xl overflow-x-scroll no-scrollbar">
      <template v-for="user in users" :key="user.id">
        <img v-if="convert(user.avatar)" :src="user.avatar" :alt="user.avatar"
          class="w-[9.6rem] lg:w-16 h-[9.6rem] lg:h-16 aspect-square rounded-[3rem] lg:rounded-2xl border-2 lg:border border-smoke/10 cursor-pointer"
          @click="createConversation(user.principal)" />
        <img v-else src="https://cdn.yshop.pl/files/RBQ8w.png" alt="default"
          class="w-[9.6rem] lg:w-16 h-[9.6rem] lg:h-16 aspect-square rounded-[3rem] lg:rounded-2xl border-2 lg:border border-smoke/10 cursor-pointer"
          @click="createConversation(user.principal)" />
      </template>
    </aside>
  </section>
</template>
