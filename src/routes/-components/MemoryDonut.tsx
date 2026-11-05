import { Cell, Pie, PieChart } from "recharts";

import { MEMORY_SEGMENTS, hatchPatternId } from "@/lib/constants";
import type { MemorySegment } from "@/lib/constants";
import { cn, formatBytes, formatPercent } from "@/lib/utils";
import type { MemoryType } from "@/domain/memory/types";

import HatchPattern from "./HatchPattern";

interface MemoryDonutProps extends React.ComponentProps<"div"> {
  memory: MemoryType;
  size?: number;
}

const HATCHED_SEGMENTS = (Object.keys(MEMORY_SEGMENTS) as MemorySegment[]).flatMap((segment) => {
  const color = MEMORY_SEGMENTS[segment].hatchColor;
  return color ? [{ segment, color }] : [];
});

export const toMemorySegments = (memory: MemoryType): { key: MemorySegment; bytes: number }[] => [
  { key: "app", bytes: memory.appBytes },
  { key: "wired", bytes: memory.wiredBytes },
  { key: "compressed", bytes: memory.compressedBytes },
  { key: "cached", bytes: memory.cachedBytes },
  { key: "free", bytes: memory.freeBytes },
];

const MemoryDonut = ({ memory, size = 124, className, ...props }: MemoryDonutProps) => {
  const segments = toMemorySegments(memory).filter((segment) => segment.bytes > 0);

  return (
    <div className={cn("relative shrink-0", className)} style={{ width: size, height: size }} {...props}>
      <PieChart width={size} height={size}>
        <defs>
          {HATCHED_SEGMENTS.map(({ segment, color }) => (
            <HatchPattern key={segment} id={hatchPatternId(segment)} color={color} />
          ))}
        </defs>
        <Pie
          data={segments}
          dataKey="bytes"
          nameKey="key"
          innerRadius={size * 0.36}
          outerRadius={size * 0.5}
          startAngle={90}
          endAngle={-270}
          paddingAngle={1.5}
          cornerRadius={3}
          stroke="none"
          isAnimationActive
          animationDuration={500}
        >
          {segments.map((segment) => (
            <Cell key={segment.key} fill={MEMORY_SEGMENTS[segment.key].color} />
          ))}
        </Pie>
      </PieChart>
      <div className="pointer-events-none absolute inset-0 flex flex-col items-center justify-center">
        <span className="tabular text-xl leading-none font-semibold">{formatPercent(memory.usedRatio)}</span>
        <span className="text-muted-foreground tabular mt-1 text-[10px]">{formatBytes(memory.usedBytes)}</span>
      </div>
    </div>
  );
};

export default MemoryDonut;
