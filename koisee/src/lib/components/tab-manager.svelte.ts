import { listen } from "@tauri-apps/api/event";

export interface MatchedImage {
  paths: [string, string];
  similarity: number;
}
export type ScanResult = {
  type: "result";
  matched_images: MatchedImage[];
};
export type ScanError = {
  type: "error";
  error: string;
};
export type ScanInProgress = {
  type: "progress";
};

type ScanIntermediate = ScanResult | ScanInProgress | ScanError;

let events: [string, ScanIntermediate][] = $state([]);

listen("scan-finished", (m) => {
  const [uuid, res] = m.payload as [string, ScanResult];
  const i = events.findIndex(([id]) => id === uuid);
  if (i !== -1)
    events[i] = [uuid, { type: "result", matched_images: res.matched_images }];
});

listen("scan-error", (m) => {
  const [uuid, err] = m.payload as [string, ScanError];
  const i = events.findIndex(([id]) => id === uuid);
  if (i !== -1) events[i] = [uuid, { type: "error", error: err.error }];
});

export function subscribe_event(uuid: string) {
  events.push([uuid, { type: "progress" }]);
}

export { events };
