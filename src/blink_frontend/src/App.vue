<script lang="ts" setup>
import NavigationBar from "@/components/navigation/NavigationBar.vue";
import type { Conversation, LastMessage, User } from "../../declarations/blink_backend/blink_backend.did";
import { RouterView } from "vue-router";
import { useAuthStore } from "@/stores/auth";
import { useStorageStore } from "@/stores/storage";
import { Principal } from "@dfinity/principal";

const auth = useAuthStore();

function verifyLogin() {
  console.log("verifylogin");
  if (auth.identity?.value === undefined || auth.identity?.value?.getPrincipal() == Principal.anonymous()) {
    throw new Error("Not logged in")
  }
}

async function logIn() {
  if (!auth.authClient) throw new Error("AuthClient not initialized");
  const storage = useStorageStore();

  await auth.logIn();

  // TODO: Get username properly
  try {
    // Set conversations
    const conversations: Conversation[] = await auth.getConversations;
    storage.setConversations(conversations);
    console.log("test:", storage.getConversations);

    // Set last messages
    const ids = conversations.map(v => v.id);
    let conversations_parsed: LastMessage[] = [];
    ids.forEach(async id => {
      conversations_parsed.push(await auth.getLastMessage(id));
    });
    storage.setLastMessages(conversations_parsed);

    // Set user
    const user: User = await auth.getUser;
    storage.setUser(user);
  } catch (e) {
    console.error(e);
  }
}

(async () => {
  await auth.setAuthClient();
  try {
    verifyLogin();
  } catch (e) {
    console.error(e);
    await logIn();
  }
})()
</script>

<template>
  <main
    class="relative aspect-[5/10] w-[28rem] 3xl:aspect-[4/9] flex flex-col items-center gap-6 p-9 rounded-3xl overflow-hidden bg-background bg-center bg-cover border border-white/10">
    <RouterView />
    <NavigationBar />
  </main>
</template>
