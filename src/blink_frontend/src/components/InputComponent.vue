<script setup lang="ts">
import { useAuthStore } from "@/stores/auth";
import { unwrap } from "@/utils/util";
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

  unwrap(await auth.actor?.send_message(BigInt(props.conversation_id), message.value));

  message.value = "";

  const conversations = [];
  if (conversations === undefined) {
    throw new Error("Failed to get conversations");
  }
}
</script>

<template>
  <aside
    class="w-full h-fit px-9 py-6 lg:px-5 lg:py-3 mt-2 lg:mt-0 mb-0 lg:mb-4 bg-smoke/5 backdrop-blur-sm rounded-2xl border-2 lg:border border-smoke/10 flex">
    <input class="text-[2rem] lg:text-base w-full bg-transparent outline-none placeholder:text-white"
      placeholder="Enter your message" type="text" />
    <button @click="send" class="text-[2rem]">→</button>
  </aside>
</template>
