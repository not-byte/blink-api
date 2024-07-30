<script setup lang="ts">
import { Actor, ActorMethod, ActorSubclass, HttpAgent, Identity } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { _SERVICE, idlFactory, Message } from "../../declarations/blink_backend/blink_backend.did.js";
import { blink_backend, canisterId, createActor } from "../../declarations/blink_backend";
import { Principal } from "@dfinity/principal";
import { ref } from "vue";
let authClient: AuthClient | null = null;
let actor: ActorSubclass<_SERVICE> | null = null;
let identity = ref<Identity | null>(null);

// import { Ref, ref } from "vue";
// let tmessages: Ref<Message[]> = ref([]);
// let my_principal = Principal.fromText("2vxsx-fae");
// console.log(my_principal);

// (async () => {
//   tmessages.value = await blink_backend.get_messages_with(Principal.fromText("ywdnu-suhio-f24aj-e5qw7-crnlw-6l4xz-nd247-e2mx5-rcqd4-xcnxe-pqe"));
// })()

async function testFn(e: any) {
  let res = await blink_backend.greet();
  console.log(res);
}

async function bootstrap() {
  authClient = await AuthClient.create();
}

async function logIn() {
  if (!authClient) throw new Error("AuthClient not initialized");

  await authClient.login({
    identityProvider: `http://dccg7-xmaaa-aaaaa-qaamq-cai.localhost:4943/`,
    // onSuccess: handleSuccess,
  });

  const identity2 = authClient.getIdentity();
  console.log("Logged in", identity2.getPrincipal());
  identity.value = identity2
}
bootstrap();
</script>

<template>
  <main>
    {{ identity?.getPrincipal() }}
    <div>
      <button @click="logIn">Log me in</button>
    </div>
    <div id="principalId"></div>

    <!-- <form action="#" @submit="handleSubmit"> -->
    <!--   <label for="name">Enter your name: &nbsp;</label> -->
    <!--   <input id="name" alt="Name" type="text" /> -->
    <!--   <button type="submit">Click Me!</button> -->
    <!-- </form> -->
    <!-- <section id="greeting">{{ greeting }}</section> -->
    <button @click="testFn">Click</button>
    <!-- <button @click="testFn2">Click2</button> -->
  </main>

  <!-- <div style="display: flex; flex-direction: column;"> -->
  <!--   <template v-for="(message, i) in tmessages" :key="i"> -->
  <!--     <div :style="[message.caller.toText() == my_principal.toText() ? 'align-self: end' : 'align-self: start']"> -->
  <!--       <template v-if="message.message.Text !== undefined"> -->
  <!--         <p>{{ message.message.Text.content }}</p> -->
  <!--       </template> -->
  <!--       <template v-else> -->
  <!--         <img :src="message.message.Image.src" width="100" height="100" :alt="message.message.Image.name"> -->
  <!--       </template> -->
  <!--     </div> -->
  <!--   </template> -->
  <!-- </div> -->
  <!--  rvkuw-pitzp-xb3bi-rz5n7-7epzj-j3mto-cncll-jstpn-nvd6q-3obt2-cae-->
</template>
