import { api, createWrapQuery } from "@/lib/api";
import { CommandEnum } from "@/lib/constants";

import { CleanCandidateListSchema, QuitAppsInputSchema, QuitResultSchema } from "./types";
import type { QuitAppsInputType } from "./types";

export const getCleanCandidates = async () =>
  createWrapQuery(async () => {
    const data = await api.call<unknown>(CommandEnum.GET_CLEAN_CANDIDATES);
    return CleanCandidateListSchema.parse(data);
  }, "get-clean-candidates");

export const quitApps = async (input: QuitAppsInputType) =>
  createWrapQuery(async () => {
    const body = QuitAppsInputSchema.parse(input);
    const data = await api.call<unknown>(CommandEnum.QUIT_APPS, { body });
    return QuitResultSchema.parse(data);
  }, "quit-apps");

export const forceQuitApps = async (input: QuitAppsInputType) =>
  createWrapQuery(async () => {
    const body = QuitAppsInputSchema.parse(input);
    const data = await api.call<unknown>(CommandEnum.FORCE_QUIT_APPS, { body });
    return QuitResultSchema.parse(data);
  }, "force-quit-apps");
