import { defineStore } from "pinia";
import type { Conversation, LastMessage, User } from "../../../declarations/blink_backend/blink_backend.did";

export const useStorageStore = defineStore("storage", {
  state: () => ({
    last_messages: [] as LastMessage[],
    conversations: [] as Conversation[],
    user: null as User | null,
  }),
  getters: {
    getConversations: (state) => { return state.conversations }
  },
  actions: {
    setUser(_user: User) {
      this.user = _user;
    },

    setLastMessages(_last_messages: LastMessage[]) {
      this.last_messages = _last_messages;
    },

    setConversations(_conversations: Conversation[]) {
      this.conversations = _conversations;
    }
  }

});
