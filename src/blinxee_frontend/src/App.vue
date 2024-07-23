<script setup>
import { ref } from 'vue';
import { blinxee_backend } from 'declarations/blinxee_backend/index';
import {AuthClient} from "@dfinity/auth-client";
import {Actor} from "@dfinity/agent";
let greeting = ref('');
let authClient = null

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  await blinxee_backend.greet(name).then((response) => {
    greeting.value = response;
  });
}

async function bootstrap() {
  authClient = await AuthClient.create()
}

function handleSuccess() {
  const principalId = authClient.getIdentity().getPrincipal().toText();

  document.getElementById(
      "principalId"
  ).innerText = `Your PrincipalId: ${principalId}`;

  Actor.agentOf(blinxee_backend).replaceIdentity(authClient.getIdentity())
}

async function logIn() {
  if (!authClient) throw new Error("AuthClient not initialized");

  const APP_NAME = "Blink APP"
  const APP_LOGO = "https://nfid.one/icons/favicon-96x96.png"

  await authClient.login({
    identityProvider: `https://nfid.one/authenticate?applicationName=${APP_NAME}&applicationLogo=${APP_LOGO}`,
    onSuccess: handleSuccess,
    windowOpenerFeatures: `
      left=${window.screen.width / 2 - 525 / 2},
      top=${window.screen.height / 2 - 705 / 2},
      toolbar=0,location=0,menubar=0,width=525,height=705
    `,
  })
}
bootstrap()
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />

    <div>
      <button @click="logIn">Log me in</button>
    </div>
    <div id="principalId"></div>

    <form action="#" @submit="handleSubmit">
      <label for="name">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>
  </main>
<!--  rvkuw-pitzp-xb3bi-rz5n7-7epzj-j3mto-cncll-jstpn-nvd6q-3obt2-cae-->
</template>

