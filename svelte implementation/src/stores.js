import { writable } from 'svelte/store';
export const wallet = writable('anon')
export const fullAddress = writable('')
export const count = writable([]);
export const length = writable('2222');
export const grid = writable(2);
export const ripEdish = writable('');
export const announcements = writable('block');
export const ripTotalCollection = writable([]);
export const chosenOne = writable([]);
export const experiment = writable([]);
export const chosenCollection = writable([]);
export const ripCollesh = writable('');
export const RipCurAddress = writable('');
export const font = writable('16px');
export const filtersChecked = writable({
  Background: "",
  Body: "",
  Eyes: "",   
  Brow: "",
  Ear: "",
  Tail: "",
  Ground: "",
  });

export const newRip = writable("none")
export const oldRip = writable("block")