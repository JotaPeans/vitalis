import { api, createWrapQuery } from "@/lib/api";
import { CommandEnum } from "@/lib/constants";

import { TemperatureGroupListSchema } from "./types";

export const getTemperatures = async () =>
  createWrapQuery(async () => {
    const data = await api.call<unknown>(CommandEnum.GET_TEMPERATURES);
    return TemperatureGroupListSchema.parse(data);
  }, "get-temperatures");
