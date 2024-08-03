<script setup lang="ts">
import { useAuthStore } from "@/stores/auth";
import { useStorageStore } from "@/stores/storage";
import { ref, Ref } from "vue";
const props = defineProps<{
  conversation_id: number;
}>();

const emit = defineEmits<{
  (e: "scroll"): void,
}>();

const auth = useAuthStore();
const storage = useStorageStore();
const message: Ref<string> = ref("");

async function send() {
  if (message.value.length == 0) {
    throw new Error("Message can't be empty");
  }

  await auth.actor?.send_message(BigInt(props.conversation_id), message.value);

  const messageTmp = message.value;
  message.value = "";

  const conversations = [];
  console.log("new_", conversations);

  if (conversations !== undefined) {
    storage.addMessage(props.conversation_id, messageTmp);
    emit("scroll");
  } else {
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
