import { z } from "zod";

import { VitalLevelSchema } from "@/lib/schemas";

export const MemorySchema = z.object({
  totalBytes: z.number(),
  usedBytes: z.number(),
  usedRatio: z.number(),
  appBytes: z.number(),
  wiredBytes: z.number(),
  compressedBytes: z.number(),
  cachedBytes: z.number(),
  freeBytes: z.number(),
  swapUsedBytes: z.number(),
  swapTotalBytes: z.number(),
  pressure: VitalLevelSchema,
});

export type MemoryType = z.infer<typeof MemorySchema>;
