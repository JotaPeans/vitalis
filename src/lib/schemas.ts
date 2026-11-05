import { z } from "zod";

export const VitalLevelSchema = z.enum(["ok", "warning", "critical"]);
