<script setup lang="ts">
import { useAuthStore } from "@/stores/auth";
import { ref, Ref } from "vue";
const props = defineProps<{
  conversation_id: number;
}>();

const auth = useAuthStore();
const message: Ref<string> = ref("");

async function send() {
  if (message.value.length == 0) {
    throw new Error("Message can't be empty");
  }

  await auth.actor?.send_message(BigInt(props.conversation_id), message.value);

  message.value = "";

  const conversations = [];
  if (conversations === undefined) {
    throw new Error("Failed to get conversations");
  }
}
</script>

<template>
  <aside class="w-full h-fit px-5 py-3 bg-white/5 backdrop-blur-sm border border-white/10 rounded-2xl flex gap-2">
    <input class="w-full bg-transparent outline-none placeholder:text-white" placeholder="What's on your mind?"
      type="text" v-model="message" />
    <button @click="send">→</button>
  </aside>
</template>
