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

export function getError(rejectText: string): { message: string } {
  // Check if it contains the JSON part and extract it
  const jsonMatch = rejectText.match(/Canister trapped explicitly:\s*(\{.*\})/);
  if (jsonMatch && jsonMatch[1]) {
    try {
      const jsonData = JSON.parse(jsonMatch[1]);
      return jsonData;
    } catch (parseError) {
      return { message: rejectText };
    }
  } else {
    return { message: rejectText };
  }
}
