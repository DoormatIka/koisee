import { type Event } from "@tauri-apps/api/event";
import { writable, type Writable } from "svelte/store";

export type Info = string;
export type ImageDecoding = string;
export type Hash = string;
export type ImageFinished = string;
export type ImageTotal = number;
export type FileError = [string, string];
export type LogError = string;

type States = {
  index: number;
  states: [ImageDecoding, Hash, ImageFinished];
};
type ProgressItems = Writable<Map<string, States>>;
export function moveToNextState(items: ProgressItems, state: string) {
  items.update((i) => {
    const item = i.get(state);
    if (!item) {
      throw new Error("AAA");
    }

    i.set(state, { ...item, index: item.index + 1 });
    return new Map(i);
  });
}
export function createState(items: ProgressItems, state: string) {
  items.update((i) => {
    i.set(state, { index: 0, states: ["1", "2", "3"] });
    return new Map(i);
  });
}

export const progress_items: ProgressItems = writable(new Map());
export let images_processed = 0;
export let images_error = 0;
export let total = 0;

export function clearProgress() {
  progress_items.set(new Map());
  images_processed = 0;
  images_error = 0;
  total = 0;
}
export function fileError(s: Event<FileError>) {
  progress_items.update((i) => {
    const [file, err] = s.payload;
    i.delete(file);
    console.log(`[ERR]: ${file} - ${err}`);
    images_error++;
    return new Map(i);
  });
}
export function finishFile(s: Event<ImageFinished>) {
  progress_items.update((i) => {
    i.delete(s.payload);
    images_processed++;
    return new Map(i);
  });
}
export function assignTotal(n: number) {
  total = n;
}

export function progressColor(index: number): string {
  // i know we can do some gradient calculation here
  // between two colors but i am lazy.
  switch (index) {
    case 0:
      return "border-gray-500 brightness-70";
    case 1:
      return "border-blue-500 brightness-100";
    case 2:
      return "border-green-300";
    default:
      return "";
  }
}
export function progressText(index: number): string {
  // i know we can do some gradient calculation here
  // between two colors but i am lazy.
  switch (index) {
    case 0:
      return "Reading";
    case 1:
      return "Hashing";
    case 2:
      return "Pushing";
    default:
      return "";
  }
}
