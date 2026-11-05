import { motion } from "motion/react";

import { cn } from "@/lib/utils";
import type { AppUsageType } from "@/domain/processes/types";

interface AppUsageBarProps extends React.ComponentProps<"div"> {
  app: AppUsageType;
  maxBytes: number;
}

const BAR_SPRING = { type: "spring", stiffness: 140, damping: 22 } as const;

const AppUsageBar = ({ app, maxBytes, className, ...props }: AppUsageBarProps) => {
  const toPercent = (bytes: number) => (maxBytes > 0 ? (bytes / maxBytes) * 100 : 0);

  return (
    <div className={cn("bg-muted flex h-1 overflow-hidden rounded-full", className)} {...props}>
      <motion.div className="bg-memory-app h-full" animate={{ width: `${toPercent(app.ownBytes)}%` }} transition={BAR_SPRING} />
      <motion.div
        className="bg-memory-app/40 h-full"
        animate={{ width: `${toPercent(app.launchedBytes)}%` }}
        transition={BAR_SPRING}
      />
    </div>
  );
};

export default AppUsageBar;
