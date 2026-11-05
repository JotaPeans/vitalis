import { z } from "zod";

import { VitalLevelSchema } from "@/lib/schemas";

export const DiskSchema = z.object({
  totalBytes: z.number(),
  availableBytes: z.number(),
  usedBytes: z.number(),
  usedRatio: z.number(),
  level: VitalLevelSchema,
});

export type DiskType = z.infer<typeof DiskSchema>;
