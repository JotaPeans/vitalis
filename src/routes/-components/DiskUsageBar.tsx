import { motion } from "motion/react";
import { HardDrive } from "lucide-react";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { useDisk } from "@/domain/disk/query";
import { cn, formatBytes, formatPercent, getVitalColorConfig } from "@/lib/utils";

const DISK_BYTE_BASE = 1000;

const DiskUsageBar = ({ className, ...props }: React.ComponentProps<"div">) => {
  const { data: disk, isPending } = useDisk();
  const colors = disk ? getVitalColorConfig(disk.level) : null;

  return (
    <Card className={cn(className)} {...props}>
      <CardHeader>
        <CardTitle className="flex items-center gap-1.5">
          <HardDrive className="size-3.5" />
          Disco
        </CardTitle>
        {disk && (
          <span className={cn("tabular text-xs font-semibold", colors?.text)}>{formatPercent(disk.usedRatio)}</span>
        )}
      </CardHeader>
      <CardContent className="flex flex-col gap-2">
        {isPending && <Skeleton className="h-10" />}
        {disk && colors && (
          <>
            <div className="bg-muted h-2.5 w-full overflow-hidden rounded-full">
              <motion.div
                className={cn("h-full rounded-full", colors.bg)}
                initial={{ width: 0 }}
                animate={{ width: `${disk.usedRatio * 100}%` }}
                transition={{ type: "spring", stiffness: 120, damping: 20 }}
              />
            </div>
            <div className="tabular flex items-center justify-between text-[11px]">
              <span className="flex items-center gap-1.5">
                <span className={cn("size-2 rounded-full", colors.bg)} />
                Em uso <strong className="font-semibold">{formatBytes(disk.usedBytes, 1, DISK_BYTE_BASE)}</strong>
              </span>
              <span className="text-muted-foreground flex items-center gap-1.5">
                <span className="bg-muted-foreground/40 size-2 rounded-full" />
                Livre <strong className="text-foreground font-semibold">{formatBytes(disk.availableBytes, 1, DISK_BYTE_BASE)}</strong>
              </span>
            </div>
          </>
        )}
      </CardContent>
    </Card>
  );
};

export default DiskUsageBar;
