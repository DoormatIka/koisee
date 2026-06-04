import { writable } from "svelte/store";

export interface RawImageData {
  path: string;
  dimensions: [number, number];
}
export interface ImageData {
  path: string;
  dimensions: [number, number];
  selected: boolean;
}
export const scanResults = writable<ImageData[][] | null>(null);
export const scanError = writable<string | null>(null);
export const scanLoading = writable<boolean>(false);
