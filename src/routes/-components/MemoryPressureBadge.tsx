import { Badge } from "@/components/ui/badge";
import { cn, getVitalColorConfig } from "@/lib/utils";
import type { VitalLevel } from "@/lib/constants";

interface MemoryPressureBadgeProps extends React.ComponentProps<"span"> {
  pressure: VitalLevel;
}

const PRESSURE_LABELS: Record<VitalLevel, string> = {
  ok: "Pressão normal",
  warning: "Pressão alta",
  critical: "Pressão crítica",
};

const MemoryPressureBadge = ({ pressure, className, ...props }: MemoryPressureBadgeProps) => {
  const colors = getVitalColorConfig(pressure);
  return (
    <Badge variant="outline" className={cn(colors.border, colors.text, className)} {...props}>
      <span className={cn("size-1.5 rounded-full", colors.bg)} />
      {PRESSURE_LABELS[pressure]}
    </Badge>
  );
};

export default MemoryPressureBadge;
