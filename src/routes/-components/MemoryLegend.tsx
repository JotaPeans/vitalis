import { MEMORY_SEGMENTS } from "@/lib/constants";
import { cn, formatBytes } from "@/lib/utils";
import type { MemoryType } from "@/domain/memory/types";

import { toMemorySegments } from "./MemoryDonut";

interface MemoryLegendProps extends React.ComponentProps<"ul"> {
  memory: MemoryType;
}

const MemoryLegend = ({ memory, className, ...props }: MemoryLegendProps) => {
  return (
    <ul className={cn("flex flex-1 flex-col gap-1.5", className)} {...props}>
      {toMemorySegments(memory).map((segment) => {
        const config = MEMORY_SEGMENTS[segment.key];
        return (
        <li
          key={segment.key}
          className={cn("flex items-center justify-between gap-2 text-[11px]", config.hatchColor && "text-muted-foreground")}
          title={config.hint}
        >
          <span className="flex items-center gap-1.5">
            <span className={cn("size-2 rounded-full", config.dot)} />
            {config.label}
          </span>
          <span className="tabular font-medium">{formatBytes(segment.bytes)}</span>
        </li>
        );
      })}
    </ul>
  );
};

export default MemoryLegend;
