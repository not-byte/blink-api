<script lang="ts" setup>
import { useAuthStore } from "@/stores/auth";
import { useStorageStore } from "@/stores/storage";
import { getError, convert } from "@/utils/util";
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
    conversation_id = Number(await auth?.actor.create_conversation([user]));
    const conversations: Conversation[] = await auth.actor?.get_user_conversations();
    storage.setConversations(conversations);
    await router.push(`/messages/${conversation_id}`);
  } catch (e) {
    let err = getError(e.message).message;
    console.error(err);
    if (err == "Conversation already exists") {
      await router.push(`/messages/${conversation_id}`);
    }
  }
}

(async () => {
  users.value = await auth?.actor.get_users();
})()
</script>

<template>
  <section class="flex flex-col gap-3">
    <!-- <h2 class="text-2xl font-semibold">Online</h2> -->
    <h2 class="text-2xl font-semibold">List of users</h2>
    <aside class="flex gap-3 overflow-x-hidden no-scrollbar">
      <template v-for="user in users" :key="user.id">
        <button @click="createConversation(user.principal)">
          <img v-if="convert(user.avatar)" :src="user.avatar" :alt="user.avatar"
            class="w-[64px] h-[64px] rounded-2xl border border-white border-opacity-5" />
          <img v-else src="https://friconix.com/png/fi-cnsuxl-user-circle.png" alt="default"
            class="w-[64px] h-[64px] rounded-2xl border border-white border-opacity-5" />
        </button>
      </template>
    </aside>
  </section>
</template>
