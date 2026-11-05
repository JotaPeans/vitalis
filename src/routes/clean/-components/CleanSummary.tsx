import { Power } from "lucide-react";

import ConfirmAction from "@/components/ConfirmAction";
import { Button } from "@/components/ui/button";
import { cn, formatBytes } from "@/lib/utils";

interface CleanSummaryProps extends React.ComponentProps<"div"> {
  selectedCount: number;
  estimatedBytes: number;
  isPending: boolean;
  onConfirm: () => void;
}

const CleanSummary = ({ selectedCount, estimatedBytes, isPending, onConfirm, className, ...props }: CleanSummaryProps) => {
  return (
    <div className={cn("flex items-center justify-between gap-3 border-t px-4 py-3", className)} {...props}>
      <div className="flex flex-col">
        <span className="tabular text-sm font-semibold">~{formatBytes(estimatedBytes)}</span>
        <span className="text-muted-foreground text-[11px]">{selectedCount} app(s) selecionado(s)</span>
      </div>
      <ConfirmAction
        title={`Encerrar ${selectedCount} app(s)?`}
        description="Cada app será fechado normalmente e pode pedir para você salvar documentos abertos."
        confirmLabel="Encerrar"
        onConfirm={onConfirm}
      >
        <Button disabled={selectedCount === 0} isLoading={isPending}>
          {!isPending && <Power />}
          {isPending ? "Encerrando…" : "Encerrar selecionados"}
        </Button>
      </ConfirmAction>
    </div>
  );
};

export default CleanSummary;
