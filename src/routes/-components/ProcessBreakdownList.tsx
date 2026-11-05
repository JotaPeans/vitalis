import { motion } from "motion/react";

import { cn, formatBytes } from "@/lib/utils";
import type { AppUsageType } from "@/domain/processes/types";

interface ProcessBreakdownListProps extends React.ComponentProps<"div"> {
  app: AppUsageType;
}

const ProcessBreakdownList = ({ app, className, ...props }: ProcessBreakdownListProps) => {
  const hiddenCount = app.processCount - app.processes.length;

  return (
    <motion.div
      initial={{ height: 0, opacity: 0 }}
      animate={{ height: "auto", opacity: 1 }}
      exit={{ height: 0, opacity: 0 }}
      transition={{ duration: 0.18, ease: "easeOut" }}
      className="overflow-hidden"
    >
      <div className={cn("bg-muted/50 mt-1.5 mb-0.5 ml-8 flex flex-col rounded-md px-2 py-1", className)} {...props}>
        {app.processes.map((process) => (
          <div key={process.pid} className="flex items-center gap-2 py-0.5 text-[11px]">
            <span
              className={cn("size-1.5 shrink-0 rounded-full", process.isOwn ? "bg-memory-app" : "bg-memory-app/40")}
            />
            <span className="min-w-0 flex-1 truncate" title={`pid ${process.pid}`}>
              {process.label}
            </span>
            <span className="tabular text-muted-foreground shrink-0">{formatBytes(process.footprintBytes)}</span>
          </div>
        ))}
        {hiddenCount > 0 && (
          <span className="text-muted-foreground py-0.5 text-[10px]">+ {hiddenCount} processo(s) menores</span>
        )}
      </div>
    </motion.div>
  );
};

export default ProcessBreakdownList;
