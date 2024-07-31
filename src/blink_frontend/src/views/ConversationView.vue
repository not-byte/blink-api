<script lang="ts" setup>
import { Conversation } from "declarations/blink_backend/blink_backend.did.js";
import { ref } from "vue";
import { useRoute } from 'vue-router'
import { useStorageStore } from "@/stores/storage";
import { useAuthStore } from "@/stores/auth";
import { storeToRefs } from "pinia";

const tmessages = ref<Conversation | undefined>();

const route = useRoute();
let id = BigInt(route.params.reciever_id as string);

const auth = useAuthStore();
const my_principal = auth.identity.getPrincipal().toText();

const storage = useStorageStore();
const { getConversation } = storeToRefs(storage);

setTimeout(() => {
  tmessages.value = getConversation.value(id);
  console.log("messages: ", tmessages.value);
}, 1000)
</script>

<template>
  <section>
    <h2>Conversation</h2>
    <p>{{ $route.params.reciever_id }}</p>

    <div style="display: flex; flex-direction: column;">
      <template v-for="(message, i) in tmessages.messages" :key="i">
        <div :style="[message.caller.toText() == my_principal ? 'align-self: end' : 'align-self: start']">
          <template v-if="message.message.Text !== undefined">
            <p>{{ message.message.Text.content }}</p>
          </template>
          <template v-else>
            <img :src="message.message.Image.src" width="100" height="100" :alt="message.message.Image.name">
          </template>
        </div>
      </template>
    </div>
  </section>
</template>
