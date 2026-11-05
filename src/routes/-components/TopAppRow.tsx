import { useState } from "react";
import { AnimatePresence, motion } from "motion/react";
import { ChevronRight } from "lucide-react";

import AppIcon from "@/components/AppIcon";
import { cn, formatBytes } from "@/lib/utils";
import type { AppUsageType } from "@/domain/processes/types";

import AppUsageBar from "./AppUsageBar";
import ProcessBreakdownList from "./ProcessBreakdownList";

interface TopAppRowProps extends React.ComponentProps<"li"> {
  app: AppUsageType;
  maxBytes: number;
}

const TopAppRow = ({ app, maxBytes, className, ...props }: TopAppRowProps) => {
  const [expanded, setExpanded] = useState(false);
  const hasLaunched = app.launchedBytes > 0;
  const canExpand = app.processes.length > 1;

  return (
    <li className={cn("py-1.5", className)} {...props}>
      <button
        type="button"
        disabled={!canExpand}
        onClick={() => setExpanded((value) => !value)}
        className="hover:bg-accent/50 -mx-1.5 flex w-[calc(100%+0.75rem)] items-center gap-2.5 rounded-md px-1.5 py-0.5 text-left transition-colors disabled:hover:bg-transparent"
      >
        <AppIcon icon={app.icon} name={app.name} />
        <div className="flex min-w-0 flex-1 flex-col gap-1">
          <div className="flex items-baseline justify-between gap-2">
            <span className="flex min-w-0 items-center gap-1 text-xs font-medium">
              <span className="truncate">{app.name}</span>
              {canExpand && (
                <motion.span animate={{ rotate: expanded ? 90 : 0 }} className="text-muted-foreground shrink-0">
                  <ChevronRight className="size-3" />
                </motion.span>
              )}
            </span>
            <span className="tabular shrink-0 text-xs font-semibold">{formatBytes(app.footprintBytes)}</span>
          </div>
          <AppUsageBar app={app} maxBytes={maxBytes} />
          {hasLaunched && (
            <span className="text-muted-foreground tabular text-[10px]">
              App {formatBytes(app.ownBytes)} · iniciados por ele {formatBytes(app.launchedBytes)}
            </span>
          )}
        </div>
      </button>
      <AnimatePresence initial={false}>{expanded && <ProcessBreakdownList app={app} />}</AnimatePresence>
    </li>
  );
};

export default TopAppRow;
