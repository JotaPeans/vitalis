import { z } from "zod";

export const SettingsSchema = z.object({
  protectedApps: z.array(z.string()),
});

export const ProtectableAppSchema = z.object({
  pid: z.number(),
  bundleId: z.string(),
  name: z.string(),
  icon: z.string().nullable(),
  isProtected: z.boolean(),
});

export const ProtectableAppListSchema = z.array(ProtectableAppSchema);

export const SetProtectedAppsInputSchema = z.object({
  bundleIds: z.array(z.string().min(1).max(255)).max(200),
});

export type SettingsType = z.infer<typeof SettingsSchema>;
export type ProtectableAppType = z.infer<typeof ProtectableAppSchema>;
export type SetProtectedAppsInputType = z.infer<typeof SetProtectedAppsInputSchema>;
