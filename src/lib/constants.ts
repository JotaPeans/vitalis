export const CommandEnum = {
  GET_TEMPERATURES: "get_temperatures",
  GET_MEMORY: "get_memory",
  GET_DISK: "get_disk",
  LIST_TOP_APPS: "list_top_apps",
  GET_CLEAN_CANDIDATES: "get_clean_candidates",
  QUIT_APPS: "quit_apps",
  FORCE_QUIT_APPS: "force_quit_apps",
  SET_PROTECTED_APPS: "set_protected_apps",
  LIST_PROTECTABLE_APPS: "list_protectable_apps",
} as const;
export type Command = (typeof CommandEnum)[keyof typeof CommandEnum];

export const EventEnum = {
  VITALS: "vitals",
} as const;

export const QueryKeyEnum = {
  TEMPERATURES: ["temperatures"],
  MEMORY: ["memory"],
  DISK: ["disk"],
  TOP_APPS: ["top-apps"],
  CLEAN_CANDIDATES: ["clean-candidates"],
  SETTINGS: ["settings"],
  PROTECTABLE_APPS: ["protectable-apps"],
  AUTOSTART: ["autostart"],
} as const;

export const VitalLevelEnum = {
  ok: "ok",
  warning: "warning",
  critical: "critical",
} as const;
export type VitalLevel = keyof typeof VitalLevelEnum;

export const MemorySegmentEnum = {
  APP: "app",
  WIRED: "wired",
  COMPRESSED: "compressed",
  CACHED: "cached",
  FREE: "free",
} as const;
export type MemorySegment = (typeof MemorySegmentEnum)[keyof typeof MemorySegmentEnum];

export const hatchPatternId = (segment: MemorySegment) => `memory-hatch-${segment}`;

interface MemorySegmentConfig {
  label: string;
  color: string;
  dot: string;
  hatchColor?: string;
  hint?: string;
}

export const MEMORY_SEGMENTS: Record<MemorySegment, MemorySegmentConfig> = {
  app: { label: "Apps", color: "var(--memory-app)", dot: "bg-memory-app" },
  wired: { label: "Sistema", color: "var(--memory-wired)", dot: "bg-memory-wired" },
  compressed: { label: "Comprimida", color: "var(--memory-compressed)", dot: "bg-memory-compressed" },
  cached: {
    label: "Cache",
    color: `url(#${hatchPatternId("cached")})`,
    dot: "bg-hatch-cached",
    hatchColor: "var(--memory-cached)",
    hint: "Arquivos guardados na RAM para abrir mais rápido. O macOS libera na hora quando um app precisa.",
  },
  free: {
    label: "Livre",
    color: `url(#${hatchPatternId("free")})`,
    dot: "bg-hatch-free",
    hatchColor: "var(--memory-free)",
  },
};

export const SECTION_STAGGER_SECONDS = 0.05;
