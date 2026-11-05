import { Thermometer } from "lucide-react";

import { Skeleton } from "@/components/ui/skeleton";
import { useTemperatures } from "@/domain/temperature/query";
import { cn } from "@/lib/utils";

import TemperatureCard from "./TemperatureCard";

const TemperatureSection = ({ className, ...props }: React.ComponentProps<"div">) => {
  const { data: groups, isPending, isError } = useTemperatures();

  return (
    <div className={cn("flex flex-col gap-2", className)} {...props}>
      <div className="text-muted-foreground flex items-center gap-1.5 px-1 text-[11px] font-semibold tracking-wide uppercase">
        <Thermometer className="size-3.5" />
        Temperatura
      </div>
      {isPending && (
        <div className="grid grid-cols-4 gap-2">
          {Array.from({ length: 4 }, (_, i) => (
            <Skeleton key={i} className="h-[62px] rounded-lg" />
          ))}
        </div>
      )}
      {isError && <p className="text-muted-foreground px-1 text-xs">Sensores de temperatura indisponíveis.</p>}
      {groups && (
        <div className="grid grid-cols-4 gap-2">
          {groups.map((group) => (
            <TemperatureCard key={group.kind} group={group} />
          ))}
        </div>
      )}
    </div>
  );
};

export default TemperatureSection;
