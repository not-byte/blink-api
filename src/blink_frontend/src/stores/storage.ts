import { defineStore } from "pinia";
import type { Conversation, LastMessage, User } from "../../../declarations/blink_backend/blink_backend.did";
import { convert } from "@/utils/util";

export const useStorageStore = defineStore("storage", {
  state: () => ({
    last_messages: [] as LastMessage[],
    conversations: [] as Conversation[],
    user: null as User | null,
  }),
  getters: {
    getLastMessages: (state) => {
      return state.last_messages.map(v => ({
        user: {
          username: v.user.username,
          avatar: convert(v.user.avatar)
        },
        content: v.content as string,
        timestamp: Number(v.timestamp),
      }));
    }
  },
  actions: {
    setUser(user: User) {
      this.user = user;
    },

    setLastMessages(last_messages: LastMessage[]) {
      this.last_messages = last_messages;
    },

    setConversations(conversations: Conversation[]) {
      this.conversations = conversations;
    }
  }
});
