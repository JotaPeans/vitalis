import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

import type { VitalLevel } from "@/lib/constants";

export interface VitalColorConfig {
  color: string;
  bg: string;
  text: string;
  border: string;
}

const VITAL_COLOR_CONFIG: Record<VitalLevel, VitalColorConfig> = {
  ok: {
    color: "var(--vital-ok)",
    bg: "bg-vital-ok",
    text: "text-vital-ok",
    border: "border-vital-ok/40",
  },
  warning: {
    color: "var(--vital-warning)",
    bg: "bg-vital-warning",
    text: "text-vital-warning",
    border: "border-vital-warning/40",
  },
  critical: {
    color: "var(--vital-critical)",
    bg: "bg-vital-critical",
    text: "text-vital-critical",
    border: "border-vital-critical/40",
  },
};

const BYTE_UNITS = ["B", "KB", "MB", "GB", "TB"];

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatBytes(bytes: number, fractionDigits = 1, base: 1000 | 1024 = 1024) {
  if (bytes <= 0) return "0 B";
  const exponent = Math.min(Math.floor(Math.log(bytes) / Math.log(base)), BYTE_UNITS.length - 1);
  const value = bytes / base ** exponent;
  const digits = exponent >= 3 ? fractionDigits : 0;
  return `${value.toLocaleString("pt-BR", { maximumFractionDigits: digits, minimumFractionDigits: digits })} ${BYTE_UNITS[exponent]}`;
}

export function formatPercent(ratio: number) {
  return `${Math.round(ratio * 100)}%`;
}

export function formatCelsius(celsius: number) {
  return `${Math.round(celsius)}°`;
}

export function getVitalColorConfig(level: VitalLevel) {
  return VITAL_COLOR_CONFIG[level];
}

export function toIconSrc(icon: string | null | undefined) {
  return icon ? `data:image/png;base64,${icon}` : undefined;
}
