import type { Result } from "@declarations/blink_backend.did";
import type { ResultConversation, ResultLastMessage, ResultText, ResultUser, ResultUserConversations, ResultWithId } from "../../../declarations/blink_backend/blink_backend.did";

export function convert<T>(input: [] | [T] | undefined): T | undefined {
  if (input === undefined) {
    return undefined;
  } else if (input.length === 0) {
    return undefined;
  } else {
    return input[0];
  }
}

export function getTime(timestamp: number): string {
  // Create a new JavaScript Date object based on the timestamp
  const date = new Date(timestamp);
  const hours = date.getHours();
  const minutes = "0" + date.getMinutes();
  const formattedTime = hours + ':' + minutes.slice(-2);

  return formattedTime;
}

export function trimStr(string: string): string {
  return string.length > 25 ? string.substring(0, 25 - 3) + "..." : string
}

export function waitFor(conditionFunction: () => boolean): Promise<void> {
  const poll = (resolve: () => void) => {
    if (conditionFunction()) resolve();
    else setTimeout(() => poll(resolve), 400);
  };

  return new Promise<void>(poll);
}

export async function sleep(millis: number) {
  return new Promise(resolve => setTimeout(resolve, millis));
}

type Results = Result | ResultText | ResultConversation | ResultLastMessage | ResultWithId | ResultUserConversations | ResultUser;
type UnwrapResult<T> = T extends { Ok: infer U } ? U : never;

export function unwrap<T extends Results>(result: T): UnwrapResult<T> {
  if (!result || typeof result !== 'object') {
    throw new Error("Invalid result: Expected an object, got undefined or non-object");
  }

  if ('Err' in result) {
    throw result.Err;
  }
  return result.Ok;
}

export function extractId(message: string): number | null {
  const parts = message.split(':');
  if (parts.length > 1) {
    return parseInt(parts[1].trim(), 10);
  }
  return null;
}
