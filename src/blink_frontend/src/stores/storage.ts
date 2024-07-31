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
        conversation_id: Number(v.conversation_id),
        user: {
          username: v.user.username,
          avatar: convert(v.user.avatar)
        },
        content: v.content as string,
        timestamp: Number(v.timestamp),
      }));
    },
    
    getConversation: (state) => {
      return (id: number) => {
        const new_conversation = state.conversations.find(v => v.id == BigInt(id))
        return {
          id: Number(new_conversation?.id),
          messages: new_conversation?.messages.map(v => ({
            id: Number(v.id),
            caller: v.caller,
            message: v.message,
            timestamp: Number(v.timestamp)
          })),
          users: new_conversation?.users,
        }
      };
    },
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
