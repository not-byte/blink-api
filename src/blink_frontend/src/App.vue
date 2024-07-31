<script lang="ts" setup>
import NavigationBar from "@/components/navigation/NavigationBar.vue";
import { RouterView } from "vue-router";
import { useAuthStore } from "@/stores/auth";
import { Principal } from "@dfinity/principal";
import { onBeforeMount } from "vue";

const auth = useAuthStore();

function verifyLogin() {
  console.log("verifylogin");
  if (auth.identity?.value === undefined || auth.identity?.value?.getPrincipal() == Principal.anonymous()) {
    throw new Error("Not logged in")
  }
}

async function logIn() {
  if (!auth.authClient) throw new Error("AuthClient not initialized");

  await auth.logIn();

  // TODO: Get username properly
  try {
    console.log(auth.getPrincipal().toText());
    let conversations = await auth.getConversations();
    console.log(conversations);
  } catch (e) {
    console.error(e);
  }
}

onBeforeMount(async () => {
  await auth.setAuthClient();
  try {
    verifyLogin();
  } catch (e) {
    console.error(e);
    await logIn();
  }
})
</script>

<template>
  <main
    class="relative aspect-[5/10] w-[28rem] 3xl:aspect-[4/9] flex flex-col items-center gap-6 p-9 rounded-3xl overflow-hidden bg-background bg-center bg-cover border border-white/10">
    <RouterView />
    <NavigationBar />
  </main>
</template>
