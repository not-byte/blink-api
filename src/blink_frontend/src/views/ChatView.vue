<script lang="ts" setup>
import HeaderComponent from "@/components/HeaderComponent.vue";
import NavigationComponent from "@/components/NavigationComponent.vue";
import InputComponent from "@/components/InputComponent.vue";
import MessageComponent from "@/components/MessageComponent.vue";
import { User } from "../../../declarations/blink_backend/blink_backend.did.js";
import type { Conversation } from "@/types/conversation";
import { Ref, ref } from "vue";
import { useRoute } from 'vue-router'
import { useStorageStore } from "@/stores/storage";
import { useAuthStore } from "@/stores/auth";
import { storeToRefs } from "pinia";
import { Principal } from "@dfinity/principal";

const conversation: Ref<Conversation | undefined> = ref();

const route = useRoute();
const id = parseInt(route.params.reciever_id as string);

const auth = useAuthStore();
const my_principal = auth.identity.getPrincipal();

const storage = useStorageStore();
const { getConversation } = storeToRefs(storage);
conversation.value = getConversation.value(id) as Conversation;

storage.$subscribe((_, _state) => {
  conversation.value = getConversation.value(id) as Conversation;
  console.log("new_conv:", getConversation.value(id));
})

function getUser(principal: Principal): string {
  if (!conversation.value) {
    return "undefined";
  }

  const user = conversation.value?.users.find((item: User) => {
    return item.principal.toText() === principal.toText();
  });

  if (user == undefined) {
    console.warn("User is undefined");
  }

  if (user?.principal.toText() == my_principal.toText()) {
    return "You";
  }

  return user?.username;
}
</script>

<template>
  <section class="w-full h-full flex flex-col gap-6 transition-root pb-12">
    <HeaderComponent :title="conversation?.name ?? ''" />
    <p>Status: Offline</p>
    <article class="flex-grow flex flex-col gap-3 rounded-xl overflow-y-scroll no-scrollbar">
      <template v-for="message in conversation?.messages ?? []" :key="message.id">
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
    <InputComponent :conversation_id="id" />
    <NavigationComponent />
  </section>
</template>
