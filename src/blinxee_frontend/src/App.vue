<script setup lang="ts">
import { Actor, HttpAgent } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { blinxee_backend, canisterId, idlFactory } from "../../declarations/blinxee_backend/index";
let authClient: AuthClient | null = null;

// async function handleSubmit(e: any) {
//   e.preventDefault();
//   const target = e.target;
//   const name = target.querySelector("#name").value;
//   await blinxee_backend.greet(name).then((response: string) => {
//     greeting.value = response;
//   });
// }

async function testFn(e: any) {
  let a = await blinxee_backend.greet();
  console.log(a);
}

async function bootstrap() {
  authClient = await AuthClient.create({
    idleOptions: {
      idleTimeout: 1000 * 60 * 30,
      disableDefaultIdleCallback: true,
    },
  });
}

async function handleSuccess() {
  const principalId = authClient?.getIdentity().getPrincipal().toText();

  document.getElementById("principalId")!.innerText = `Your PrincipalId: ${principalId}`;
}

async function logIn() {
  if (!authClient) throw new Error("AuthClient not initialized");

  const APP_NAME = "Blink APP";
  const APP_LOGO = "https://nfid.one/icons/favicon-96x96.png";

  await authClient.login({
    identityProvider: `https://nfid.one/authenticate?applicationName=${APP_NAME}&applicationLogo=${APP_LOGO}`,
    onSuccess: handleSuccess,
    windowOpenerFeatures: `
      left=${window.screen.width / 2 - 525 / 2},
      top=${window.screen.height / 2 - 705 / 2},
      toolbar=0,location=0,menubar=0,width=525,height=705
    `,
  });

  const identity = authClient?.getIdentity();
  const actor = Actor.createActor(idlFactory, {
    agent: await HttpAgent.create({ identity }),
    canisterId,
  });

  Actor.agentOf(actor)!.replaceIdentity!(authClient?.getIdentity()!);
}
bootstrap();
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

    <!-- <form action="#" @submit="handleSubmit"> -->
    <!--   <label for="name">Enter your name: &nbsp;</label> -->
    <!--   <input id="name" alt="Name" type="text" /> -->
    <!--   <button type="submit">Click Me!</button> -->
    <!-- </form> -->
    <!-- <section id="greeting">{{ greeting }}</section> -->
    <button @click="testFn">Click</button>
  </main>
  <!--  rvkuw-pitzp-xb3bi-rz5n7-7epzj-j3mto-cncll-jstpn-nvd6q-3obt2-cae-->
</template>
