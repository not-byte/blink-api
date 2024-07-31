<script lang="ts" setup>
import { Conversation } from "declarations/blink_backend/blink_backend.did.js";
import { ref } from "vue";
import { useRoute } from 'vue-router'
import { AuthClient } from "@dfinity/auth-client";
import { Principal } from "@dfinity/principal";
import { useAuthStore } from "@/stores/auth";
let tmessages = ref<Conversation | undefined>();

const route = useRoute();
let id = BigInt(route.params.reciever_id as string);

const auth = useAuthStore();

(async () => {
  console.log(auth.getPrincipal());
  tmessages.value = await auth.actor.get_messages(id);
})()
</script>

<template>
  <section>
    <h2>Conversation</h2>
    <p>{{ $route.params.reciever_id }}</p>

    <div style="display: flex; flex-direction: column;">
      <template v-for="(message, i) in tmessages" :key="i">
        <div :style="[message.caller.toText() == my_principal.toText() ? 'align-self: end' : 'align-self: start']">
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
