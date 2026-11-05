import { api, createWrapQuery } from "@/lib/api";
import { CommandEnum } from "@/lib/constants";

import { DiskSchema } from "./types";

export const getDisk = async () =>
  createWrapQuery(async () => {
    const data = await api.call<unknown>(CommandEnum.GET_DISK);
    return DiskSchema.parse(data);
  }, "get-disk");
