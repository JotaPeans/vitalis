import { z } from "zod";

import { VitalLevelSchema } from "@/lib/schemas";

export const TemperatureKindSchema = z.enum(["cpu", "gpu", "ssd", "battery"]);

export const TemperatureGroupSchema = z.object({
  kind: TemperatureKindSchema,
  label: z.string(),
  maxCelsius: z.number(),
  averageCelsius: z.number(),
  sensorCount: z.number(),
  level: VitalLevelSchema,
});

export const TemperatureGroupListSchema = z.array(TemperatureGroupSchema);

export type TemperatureKindType = z.infer<typeof TemperatureKindSchema>;
export type TemperatureGroupType = z.infer<typeof TemperatureGroupSchema>;
