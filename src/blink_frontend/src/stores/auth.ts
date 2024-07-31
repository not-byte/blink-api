import { ref } from "vue";
import { defineStore } from "pinia";
import type { ActorSubclass, Identity } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import type { _SERVICE, Conversation, LastMessage } from "../../../declarations/blink_backend/blink_backend.did.js";
import { canisterId, createActor } from "../../../declarations/blink_backend";
import type { Principal } from "@dfinity/principal";

function convert<T>(input: [] | [T] | undefined): T | undefined {
  if (input === undefined) {
    return undefined;
  } else if (input.length === 0) {
    return undefined;
  } else {
    return input[0];
  }
}

export const useAuthStore = defineStore("auth", () => {
  const identity = ref<Identity>();
  const authClient = ref<AuthClient>();
  const actor = ref<ActorSubclass<_SERVICE>>();

  async function setAuthClient() {
    console.log("set auth client");
    authClient.value = await AuthClient.create();
    console.log("set auth client end");
  }

  function setIdentity() {
    console.log("set identity");
    identity.value = authClient.value?.getIdentity()
    console.log("set identity end");
  }

  function setActor() {
    console.log("set actor");
    actor.value = createActor(canisterId, {
      agentOptions: {
        identity: identity.value
      }
    });
    console.log("set actor end");
  }

  async function logIn() {
    // FIX: A stupid workaround
    const localAuthClient = await AuthClient.create();
    await localAuthClient.login({
      identityProvider: `http://dccg7-xmaaa-aaaaa-qaamq-cai.localhost:4943/`,
      onSuccess: () => {
        console.log('Login Successful!');
      },
      onError: (error) => {
        console.error('Login Failed: ', error);
      }
    });
    console.log(localAuthClient.getIdentity().getPrincipal().toText());
    identity.value = localAuthClient.getIdentity();
    actor.value = createActor(canisterId, {
      agentOptions: {
        identity: identity.value
      }
    });
    authClient.value = localAuthClient;
    console.log("login end");
  }

  async function addUser(name: string, avatar?: string) {
    console.log("user start");
    const new_avatar: [string] | [] = avatar !== undefined ? [avatar] : [];
    await actor.value?.add_user(name, new_avatar)
    console.log("user end");
  }

  async function getConversations(): Promise<Conversation[] | undefined> {
    return await actor.value?.get_user_conversations();
  }

  async function getLastMessage(id: number): Promise<LastMessage | undefined> {
    return convert(await actor.value?.get_last_message(BigInt(id)));
  }

  function getPrincipal(): Principal | undefined {
    return identity.value?.getPrincipal();
  }

  return { identity, authClient, actor, setAuthClient, setIdentity, setActor, logIn, addUser, getPrincipal, getConversations, getLastMessage };
});
