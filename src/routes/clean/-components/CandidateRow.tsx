import { motion } from "motion/react";

import AppIcon from "@/components/AppIcon";
import { Badge } from "@/components/ui/badge";
import { Checkbox } from "@/components/ui/checkbox";
import { cn, formatBytes } from "@/lib/utils";
import type { CleanCandidateType } from "@/domain/cleaner/types";

interface CandidateRowProps extends React.ComponentProps<"label"> {
  candidate: CleanCandidateType;
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
}

const CandidateRow = ({ candidate, checked, onCheckedChange, className, ...props }: CandidateRowProps) => {
  return (
    <motion.li layout initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
      <label
        className={cn(
          "hover:bg-accent/60 flex cursor-pointer items-center gap-2.5 rounded-lg px-2 py-2 transition-colors",
          checked && "bg-accent/40",
          className,
        )}
        {...props}
      >
        <Checkbox checked={checked} onCheckedChange={(value) => onCheckedChange(value === true)} />
        <AppIcon icon={candidate.icon} name={candidate.name} />
        <div className="flex min-w-0 flex-1 flex-col gap-0.5">
          <span className="truncate text-xs font-medium">{candidate.name}</span>
          <div className="flex gap-1">
            {candidate.recommended && <Badge>Sugerido</Badge>}
            {candidate.isHidden && <Badge variant="secondary">Oculto</Badge>}
            {!candidate.hasVisibleWindow && <Badge variant="secondary">Sem janelas</Badge>}
          </div>
        </div>
        <span className="tabular shrink-0 text-xs font-semibold">{formatBytes(candidate.footprintBytes)}</span>
      </label>
    </motion.li>
  );
};

export default CandidateRow;
