import { z } from "zod";

export const CleanCandidateSchema = z.object({
  pid: z.number(),
  name: z.string(),
  bundleId: z.string().nullable(),
  footprintBytes: z.number(),
  icon: z.string().nullable(),
  hasVisibleWindow: z.boolean(),
  isHidden: z.boolean(),
  recommended: z.boolean(),
});

export const CleanCandidateListSchema = z.array(CleanCandidateSchema);

export const QuitOutcomeSchema = z.object({
  pid: z.number(),
  name: z.string(),
  footprintBytes: z.number(),
});

export const QuitResultSchema = z.object({
  closed: z.array(QuitOutcomeSchema),
  refused: z.array(QuitOutcomeSchema),
  freedBytes: z.number(),
});

export const QuitAppsInputSchema = z.object({
  pids: z.array(z.number().int().positive()).min(1, "Selecione ao menos um app"),
});

export type CleanCandidateType = z.infer<typeof CleanCandidateSchema>;
export type QuitOutcomeType = z.infer<typeof QuitOutcomeSchema>;
export type QuitResultType = z.infer<typeof QuitResultSchema>;
export type QuitAppsInputType = z.infer<typeof QuitAppsInputSchema>;
