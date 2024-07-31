import { ref } from "vue";
import { defineStore } from "pinia";
import type { ActorSubclass, Identity } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import type { _SERVICE, Conversation, LastMessage, User } from "../../../declarations/blink_backend/blink_backend.did.js";
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

export const useAuthStore = defineStore("auth", {
  state: () => ({
    identity: null as Identity | null,
    authClient: null as AuthClient | null,
    actor: null as ActorSubclass<_SERVICE> | null,
  }),
  getters: {
    getConversations: async (state) => {
      console.log(state.actor);
      console.log(state.actor?.get_user_conversations());
      return await state.actor?.get_user_conversations();
    },

    getLastMessage: async (state) => {
      return async (id: number) => convert(await state.actor?.get_last_message(BigInt(id)));
    },

    getPrincipal: (state) => {
      return state.identity?.getPrincipal();
    },

    getUser: async (state) => {
      return convert(await state.actor?.get_user());
    },
  },
  actions: {
    async setAuthClient() {
      console.log("set auth client");
      this.authClient = await AuthClient.create();
      console.log("set auth client end");
    },

    setIdentity() {
      console.log("set identity");
      this.identity = this.authClient?.getIdentity() as Identity
      console.log("set identity end");
    },

    async logIn() {
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
      this.identity = localAuthClient.getIdentity();
      this.authClient = localAuthClient;
      this.actor = createActor(canisterId, {
        agentOptions: {
          identity: this.identity as Identity
        }
      })
      console.log("login end");
    },

    // async addUser(name: string, avatar?: string) {
    //   console.log("user start");
    //   const new_avatar: [string] | [] = avatar !== undefined ? [avatar] : [];
    //   const actor = getActor();
    //   await actor.add_user(name, new_avatar)
    //   console.log("user end");
    // },
  }
});


