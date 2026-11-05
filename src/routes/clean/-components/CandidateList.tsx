import { AnimatePresence } from "motion/react";
import { Info, PartyPopper } from "lucide-react";

import { Skeleton } from "@/components/ui/skeleton";
import { cn } from "@/lib/utils";
import type { CleanCandidateType } from "@/domain/cleaner/types";

import CandidateRow from "./CandidateRow";

interface CandidateListProps extends React.ComponentProps<"div"> {
  candidates: CleanCandidateType[] | undefined;
  isPending: boolean;
  selected: Set<number>;
  onSelectionChange: (pid: number, checked: boolean) => void;
}

const CandidateList = ({ candidates, isPending, selected, onSelectionChange, className, ...props }: CandidateListProps) => {
  return (
    <div className={cn("flex flex-col gap-2 px-4 pb-3", className)} {...props}>
      <div className="bg-muted/60 text-muted-foreground flex gap-2 rounded-lg p-2.5 text-[11px] leading-relaxed">
        <Info className="mt-0.5 size-3.5 shrink-0" />
        <span>
          O app em uso, o Finder e seus apps protegidos não aparecem. O cache do sistema é liberado pelo macOS
          automaticamente quando necessário.
        </span>
      </div>
      {isPending &&
        Array.from({ length: 5 }, (_, i) => <Skeleton key={i} className="h-11 rounded-lg" />)}
      {candidates?.length === 0 && (
        <div className="text-muted-foreground flex flex-col items-center gap-2 py-10 text-xs">
          <PartyPopper className="size-6" />
          Nenhum app para encerrar agora.
        </div>
      )}
      <ul className="flex flex-col">
        <AnimatePresence initial={false}>
          {candidates?.map((candidate) => (
            <CandidateRow
              key={candidate.pid}
              candidate={candidate}
              checked={selected.has(candidate.pid)}
              onCheckedChange={(checked) => onSelectionChange(candidate.pid, checked)}
            />
          ))}
        </AnimatePresence>
      </ul>
    </div>
  );
};

export default CandidateList;
