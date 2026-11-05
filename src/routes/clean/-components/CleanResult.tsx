import { motion } from "motion/react";
import { CircleCheck, Skull } from "lucide-react";

import ConfirmAction from "@/components/ConfirmAction";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { cn, formatBytes } from "@/lib/utils";
import type { QuitResultType } from "@/domain/cleaner/types";

import QuitOutcomeList from "./QuitOutcomeList";

interface CleanResultProps extends React.ComponentProps<"div"> {
  result: QuitResultType;
  isForcing: boolean;
  onForceQuit: (pids: number[]) => void;
  onDone: () => void;
}

const CleanResult = ({ result, isForcing, onForceQuit, onDone, className, ...props }: CleanResultProps) => {
  const refusedPids = result.refused.map((outcome) => outcome.pid);

  return (
    <div className={cn("flex flex-col gap-3 px-4 pb-3", className)} {...props}>
      <motion.div
        initial={{ scale: 0.9, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        transition={{ type: "spring", stiffness: 260, damping: 20 }}
        className="flex flex-col items-center gap-1 py-4 text-center"
      >
        <CircleCheck className="text-vital-ok size-9" />
        <span className="tabular text-2xl font-semibold">{formatBytes(result.freedBytes)}</span>
        <span className="text-muted-foreground text-xs">liberados de memória em uso</span>
      </motion.div>
      {result.closed.length > 0 && (
        <Card>
          <CardContent>
            <QuitOutcomeList title="Encerrados" outcomes={result.closed} />
          </CardContent>
        </Card>
      )}
      {result.refused.length > 0 && (
        <Card className="border-vital-warning/40">
          <CardContent className="flex flex-col gap-3">
            <QuitOutcomeList
              title="Ainda abertos"
              outcomes={result.refused}
              classNames={{ title: "text-vital-warning" }}
            />
            <p className="text-muted-foreground text-[11px] leading-relaxed">
              Esses apps podem estar esperando você salvar algo. Forçar o encerramento descarta alterações não salvas.
            </p>
            <ConfirmAction
              title="Forçar encerramento?"
              description="Alterações não salvas nesses apps serão perdidas. Essa ação não pode ser desfeita."
              confirmLabel="Forçar encerramento"
              destructive
              onConfirm={() => onForceQuit(refusedPids)}
            >
              <Button variant="destructive" size="sm" isLoading={isForcing}>
                {!isForcing && <Skull />}
                Forçar encerramento
              </Button>
            </ConfirmAction>
          </CardContent>
        </Card>
      )}
      <Button variant="outline" onClick={onDone}>
        Concluir
      </Button>
    </div>
  );
};

export default CleanResult;
