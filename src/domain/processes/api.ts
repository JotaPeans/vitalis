import { api, createWrapQuery } from "@/lib/api";
import { CommandEnum } from "@/lib/constants";

import { AppUsageListSchema } from "./types";

export const listTopApps = async () =>
  createWrapQuery(async () => {
    const data = await api.call<unknown>(CommandEnum.LIST_TOP_APPS);
    return AppUsageListSchema.parse(data);
  }, "list-top-apps");
