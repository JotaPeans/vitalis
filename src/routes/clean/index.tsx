import { useMemo, useState } from "react";
import { createFileRoute, useNavigate } from "@tanstack/react-router";

import PageHeader from "@/components/PageHeader";
import { ScrollArea } from "@/components/ui/scroll-area";
import { useCleanCandidates, useForceQuitAppsMutation, useQuitAppsMutation } from "@/domain/cleaner/query";
import type { CleanCandidateType, QuitResultType } from "@/domain/cleaner/types";

import CandidateList from "./-components/CandidateList";
import CleanResult from "./-components/CleanResult";
import CleanSummary from "./-components/CleanSummary";

export const Route = createFileRoute("/clean/")({
  component: CleanPage,
});

const recommendedPids = (candidates: CleanCandidateType[] | undefined) =>
  new Set((candidates ?? []).filter((candidate) => candidate.recommended).map((candidate) => candidate.pid));

function CleanPage() {
  const navigate = useNavigate();
  const { data: candidates, isPending } = useCleanCandidates();
  const quitMutation = useQuitAppsMutation();
  const forceQuitMutation = useForceQuitAppsMutation();
  const [selection, setSelection] = useState<Set<number> | null>(null);
  const [result, setResult] = useState<QuitResultType | null>(null);

  const selected = useMemo(() => {
    const available = new Set((candidates ?? []).map((candidate) => candidate.pid));
    const base = selection ?? recommendedPids(candidates);
    return new Set([...base].filter((pid) => available.has(pid)));
  }, [candidates, selection]);

  const estimatedBytes = (candidates ?? [])
    .filter((candidate) => selected.has(candidate.pid))
    .reduce((total, candidate) => total + candidate.footprintBytes, 0);

  const toggle = (pid: number, checked: boolean) => {
    const next = new Set(selected);
    if (checked) next.add(pid);
    else next.delete(pid);
    setSelection(next);
  };

  const quitSelected = () => {
    quitMutation.mutate({ pids: [...selected] }, { onSuccess: setResult });
  };

  const forceQuit = (pids: number[]) => {
    forceQuitMutation.mutate({ pids }, { onSuccess: setResult });
  };

  return (
    <>
      <PageHeader title="Limpar memória" subtitle="Escolha os apps que podem ser encerrados" backTo="/" />
      <ScrollArea className="min-h-0 flex-1">
        {result ? (
          <CleanResult
            result={result}
            isForcing={forceQuitMutation.isPending}
            onForceQuit={forceQuit}
            onDone={() => navigate({ to: "/" })}
          />
        ) : (
          <CandidateList candidates={candidates} isPending={isPending} selected={selected} onSelectionChange={toggle} />
        )}
      </ScrollArea>
      {!result && (
        <CleanSummary
          selectedCount={selected.size}
          estimatedBytes={estimatedBytes}
          isPending={quitMutation.isPending}
          onConfirm={quitSelected}
        />
      )}
    </>
  );
}
