<script lang="ts" setup>
import { storeToRefs } from "pinia";
import type { Conversation, LastMessage, User } from "@declarations/blink_backend.did";
import { RouterView } from "vue-router";
import { useAuthStore } from "@/stores/auth";
import { useStorageStore } from "@/stores/storage";
import { Principal } from "@dfinity/principal";
import { getError, waitFor } from "@/utils/util";

const auth = useAuthStore();
const storage = useStorageStore();
const { getLastMessage } = storeToRefs(auth);

function verifyLogin() {
  console.info("Verifing login...");
  if (auth.identity === undefined || auth.identity.getPrincipal() == Principal.anonymous()) {
    throw new Error("Not logged in")
  }
}

async function logIn() {
  if (!auth.authClient) throw new Error("AuthClient not initialized");

  if (auth.isAnonymous) {
    await auth.logIn();
  }

  // Wait until User will log in
  await waitFor(() => auth.isAnonymous === false);

  try {
    update();
  } catch (e) {
    console.error(getError(e).message);
  }
}

async function update() {
  // Set conversations
  // NOTE: For some reason this doesn't work?
  const _conversations: Conversation[] = await auth.getConversations;
  // NOTE: But works if fetched twice?
  const conversations: Conversation[] = await auth.actor?.get_user_conversations();
  storage.setConversations(conversations);

  // Set last messages
  const ids = conversations.map(v => v.id);
  let conversations_parsed: LastMessage[] = [];
  conversations_parsed = await Promise.all(ids.map(async id => {
    return await getLastMessage.value(id);
  }));
  storage.setLastMessages(conversations_parsed);

  // Set user
  const user: User = await auth.getUser;
  storage.setUser(user);
}

setInterval(async () => {
  update();
}, 2000);

(async () => {
  await auth.setAuthClient();
  try {
    verifyLogin();
  } catch {
    console.error("Not logged in");
    await logIn();
  }
})()
</script>

<template>
  <main
    class="relative w-[28rem] h-[63rem] aspect-[4/9] p-9 rounded-3xl overflow-clip bg-base bg-center bg-cover border border-white/10">
    <router-view v-slot="{ Component }">
      <transition mode="out-in">
        <component :is="Component" />
      </transition>
    </router-view>
  </main>
</template>

<style>
.v-enter-active,
.v-leave-active {
  transition: opacity 700ms ease-in-out;
}

.v-enter-active {
  transition-delay: 350ms;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>
