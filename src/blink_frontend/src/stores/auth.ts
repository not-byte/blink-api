import { defineStore } from "pinia";
import type { ActorSubclass, Identity } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import type { _SERVICE, User } from "../../../declarations/blink_backend/blink_backend.did.js";
import { canisterId, createActor } from "../../../declarations/blink_backend";
import { convert } from "@/utils/util";

export const useAuthStore = defineStore("auth", {
  state: () => ({
    identity: null as Identity | null,
    authClient: null as AuthClient | null,
    actor: null as ActorSubclass<_SERVICE> | null,
  }),
  getters: {
    getConversations: async (state) => await state.actor?.get_user_conversations(),

    getLastMessage: (state) => {
      return async (id: number) => convert(await state.actor?.get_last_message(BigInt(id)));
    },

    getPrincipal: (state) => {
      return state.identity?.getPrincipal();
    },

    async getUser(): Promise<User | undefined> {
      return convert(await this.actor?.get_user());
    }
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

    async addUser(name: string, avatar?: string) {
      const new_avatar: [string] | [] = avatar !== undefined ? [avatar] : [];
      await this.actor?.add_user(name, new_avatar)
    },
  }
});


