import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart";

import { api, createWrapQuery } from "@/lib/api";
import { CommandEnum } from "@/lib/constants";

import { ProtectableAppListSchema, SetProtectedAppsInputSchema, SettingsSchema } from "./types";
import type { SetProtectedAppsInputType } from "./types";

export const setProtectedApps = async (input: SetProtectedAppsInputType) =>
  createWrapQuery(async () => {
    const body = SetProtectedAppsInputSchema.parse(input);
    const data = await api.call<unknown>(CommandEnum.SET_PROTECTED_APPS, { body });
    return SettingsSchema.parse(data);
  }, "set-protected-apps");

export const listProtectableApps = async () =>
  createWrapQuery(async () => {
    const data = await api.call<unknown>(CommandEnum.LIST_PROTECTABLE_APPS);
    return ProtectableAppListSchema.parse(data);
  }, "list-protectable-apps");

export const getAutostart = async () => createWrapQuery(() => isEnabled(), "get-autostart");

export const setAutostart = async (enabled: boolean) =>
  createWrapQuery(async () => {
    await (enabled ? enable() : disable());
    return enabled;
  }, "set-autostart");
