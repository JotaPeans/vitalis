import { z } from "zod";

export const ProcessBreakdownSchema = z.object({
  pid: z.number(),
  label: z.string(),
  footprintBytes: z.number(),
  isOwn: z.boolean(),
});

export const AppUsageSchema = z.object({
  pid: z.number(),
  name: z.string(),
  bundleId: z.string().nullable(),
  footprintBytes: z.number(),
  ownBytes: z.number(),
  launchedBytes: z.number(),
  processCount: z.number(),
  processes: z.array(ProcessBreakdownSchema),
  icon: z.string().nullable(),
});

export const AppUsageListSchema = z.array(AppUsageSchema);

export type ProcessBreakdownType = z.infer<typeof ProcessBreakdownSchema>;
export type AppUsageType = z.infer<typeof AppUsageSchema>;
