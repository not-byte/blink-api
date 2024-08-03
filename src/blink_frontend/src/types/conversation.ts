import type { Principal } from "@dfinity/principal";
import type { MessageContent, User } from "../../../declarations/blink_backend/blink_backend.did";

export interface Message {
  id: number;
  caller: Principal;
  message: MessageContent;
  timestamp: number;
}

export interface Conversation {
  id: number;
  name: string;
  messages: Message[];
  users: User[];
}
