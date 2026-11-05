import { listen, type UnlistenFn } from "@tauri-apps/api/event";

import { EventEnum } from "@/lib/constants";

import { VitalsSchema, type VitalsEventType } from "./types";

export const subscribeVitals = (onVitals: (vitals: VitalsEventType) => void): Promise<UnlistenFn> =>
  listen<unknown>(EventEnum.VITALS, (event) => {
    const parsed = VitalsSchema.safeParse(event.payload);
    if (parsed.success) onVitals(parsed.data);
    else console.error("~ vitals event error:", parsed.error.message);
  });
