import { defineStore } from "pinia";
import type { Conversation, LastMessage, User, Message } from "../../../declarations/blink_backend/blink_backend.did";
import { convert } from "@/utils/util";

export const useStorageStore = defineStore("storage", {
  state: () => ({
    last_messages: [] as LastMessage[],
    conversations: [] as Conversation[],
    user: null as User | null,
  }),
  getters: {
    getLastMessages: (state) => {
      console.warn(state.last_messages);
      return state.last_messages.map((v: LastMessage) => ({
        conversation_id: Number(v.conversation_id),
        user: {
          principal: v.user.principal,
          username: v.user.username,
          avatar: convert(v.user.avatar),
        },
        content: v.content as string,
        timestamp: Number(v.timestamp),
      }));
    },
    
    getConversation: (state) => {
      return (id: number) => {
        const new_conversation = state.conversations.find((conversation: Conversation) => conversation.id == BigInt(id))
        console.log(new_conversation);
        return {
          id: Number(new_conversation?.id),
          name: new_conversation?.name,
          messages: new_conversation?.messages.map((message: Message) => ({
            id: Number(message.id),
            caller: message.caller,
            message: message.message,
            timestamp: Number(message.timestamp)
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
