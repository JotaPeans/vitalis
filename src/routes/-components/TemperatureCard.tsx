import { Battery, Cpu, Gpu, HardDrive } from "lucide-react";
import type { LucideIcon } from "lucide-react";

import { cn, formatCelsius, getVitalColorConfig } from "@/lib/utils";
import type { TemperatureGroupType, TemperatureKindType } from "@/domain/temperature/types";

interface TemperatureCardProps extends React.ComponentProps<"div"> {
  group: TemperatureGroupType;
}

const KIND_ICONS: Record<TemperatureKindType, LucideIcon> = {
  cpu: Cpu,
  gpu: Gpu,
  ssd: HardDrive,
  battery: Battery,
};

const TemperatureCard = ({ group, className, ...props }: TemperatureCardProps) => {
  const Icon = KIND_ICONS[group.kind];
  const colors = getVitalColorConfig(group.level);
  const showAverage = group.sensorCount > 1;

  return (
    <div
      className={cn("bg-card flex flex-col gap-1 rounded-lg border px-2.5 py-2", className)}
      title={`${group.sensorCount} sensor(es) · média ${formatCelsius(group.averageCelsius)}`}
      {...props}
    >
      <div className="text-muted-foreground flex items-center gap-1 text-[10px] font-medium">
        <Icon className="size-3" />
        <span className="truncate">{group.label}</span>
      </div>
      <div className="flex items-baseline gap-1">
        <span className={cn("tabular text-lg leading-none font-semibold", colors.text)}>
          {formatCelsius(group.maxCelsius)}
        </span>
        <span className={cn("size-1.5 rounded-full", colors.bg)} />
      </div>
      <span className="text-muted-foreground tabular text-[10px]">
        {showAverage ? `média ${formatCelsius(group.averageCelsius)}` : "1 sensor"}
      </span>
    </div>
  );
};

export default TemperatureCard;
