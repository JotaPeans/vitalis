import { api, createWrapQuery } from "@/lib/api";
import { CommandEnum } from "@/lib/constants";

import { MemorySchema } from "./types";

export const getMemory = async () =>
  createWrapQuery(async () => {
    const data = await api.call<unknown>(CommandEnum.GET_MEMORY);
    return MemorySchema.parse(data);
  }, "get-memory");
