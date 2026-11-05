import { MemoryStick } from "lucide-react";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { useMemory } from "@/domain/memory/query";
import { cn, formatBytes } from "@/lib/utils";

import MemoryDonut from "./MemoryDonut";
import MemoryLegend from "./MemoryLegend";
import MemoryPressureBadge from "./MemoryPressureBadge";

const MemoryCard = ({ className, ...props }: React.ComponentProps<"div">) => {
  const { data: memory, isPending } = useMemory();

  return (
    <Card className={cn(className)} {...props}>
      <CardHeader>
        <CardTitle className="flex items-center gap-1.5">
          <MemoryStick className="size-3.5" />
          Memória
        </CardTitle>
        {memory && <MemoryPressureBadge pressure={memory.pressure} />}
      </CardHeader>
      <CardContent className="flex flex-col gap-3">
        {isPending && <Skeleton className="h-[124px]" />}
        {memory && (
          <>
            <div className="flex items-center gap-4">
              <MemoryDonut memory={memory} />
              <MemoryLegend memory={memory} />
            </div>
            <div className="text-muted-foreground tabular flex justify-between border-t pt-2 text-[11px]">
              <span>
                Total <strong className="text-foreground font-semibold">{formatBytes(memory.totalBytes, 0)}</strong>
              </span>
              <span>
                Disponível{" "}
                <strong className="text-foreground font-semibold">
                  {formatBytes(memory.freeBytes + memory.cachedBytes)}
                </strong>
              </span>
              <span>
                Swap <strong className="text-foreground font-semibold">{formatBytes(memory.swapUsedBytes)}</strong>
              </span>
            </div>
          </>
        )}
      </CardContent>
    </Card>
  );
};

export default MemoryCard;
