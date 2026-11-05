import { z } from "zod";

import { DiskSchema } from "@/domain/disk/types";
import { MemorySchema } from "@/domain/memory/types";
import { AppUsageListSchema } from "@/domain/processes/types";
import { TemperatureGroupListSchema } from "@/domain/temperature/types";

export const VitalsSchema = z.object({
  temperatures: TemperatureGroupListSchema,
  memory: MemorySchema.nullable(),
  disk: DiskSchema.nullable(),
  topApps: AppUsageListSchema,
});

export type VitalsEventType = z.infer<typeof VitalsSchema>;
