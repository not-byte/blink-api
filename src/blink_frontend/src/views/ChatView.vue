<script lang="ts" setup>
import HeaderComponent from "@/components/HeaderComponent.vue";
import NavigationComponent from "@/components/NavigationComponent.vue";
import InputComponent from "@/components/InputComponent.vue";
import MessageComponent from "@/components/MessageComponent.vue";
import { Conversation, User } from "../../../declarations/blink_backend/blink_backend.did.js";
import { ref } from "vue";
import { useRoute } from 'vue-router'
import { useStorageStore } from "@/stores/storage";
import { useAuthStore } from "@/stores/auth";
import { storeToRefs } from "pinia";
import { Principal } from "@dfinity/principal";

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

function getUser(principal: Principal): string {
  const user = tmessages.value.users.find((item: User) => {
    item.principal == principal
  });

  if (user.principal == my_principal) {
    return "You";
  }

  return user.username;
}
</script>

<template>
  <section class="w-full h-full flex flex-col gap-6 transition-root pb-12">
    <HeaderComponent :title="$route.params.reciever_id" />
    <p>Status: Offline</p>
    <article class="flex-grow flex flex-col gap-3 rounded-xl overflow-y-scroll no-scrollbar">
      <template v-for="message in tmessages.messages" :key="message.id">
        <template v-if="message.message.Text">
          <MessageComponent :message="message.message.Text.content" :sender="getUser(message.caller)"
            :timestamp="message.timestamp" />
        </template>
        <template v-else>
          <MessageComponent message="[Images are not yet implemented]" :sender="getUser(message.caller)"
            :timestamp="message.timestamp" />
        </template>
      </template>
    </article>
    <InputComponent />
    <NavigationComponent />
  </section>
</template>
<!-- <MessageComponent message="Brahhhhh hehe" sender="You" /> -->
<!-- <MessageComponent :message="message" sender="You" /> -->
