import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";

import { QueryKeyEnum } from "@/lib/constants";
import { formatBytes } from "@/lib/utils";

import { forceQuitApps, getCleanCandidates, quitApps } from "./api";
import type { QuitAppsInputType, QuitResultType } from "./types";

export const useCleanCandidates = () =>
  useQuery({
    queryKey: QueryKeyEnum.CLEAN_CANDIDATES,
    queryFn: async () => {
      const { data, error } = await getCleanCandidates();
      if (error) throw error;
      return data;
    },
    staleTime: 0,
    refetchOnMount: "always",
  });

const notifyResult = (result: QuitResultType) => {
  if (result.closed.length === 0) {
    toast.warning("Nenhum app foi encerrado");
    return;
  }
  toast.success(`${result.closed.length} app(s) encerrado(s) · ${formatBytes(result.freedBytes)} liberados`);
};

const useQuitMutation = (mutate: typeof quitApps) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (input: QuitAppsInputType) => {
      const { data, error } = await mutate(input);
      if (error) throw error;
      return data;
    },
    onSuccess: (result) => {
      queryClient.invalidateQueries({ queryKey: QueryKeyEnum.CLEAN_CANDIDATES });
      queryClient.invalidateQueries({ queryKey: QueryKeyEnum.MEMORY });
      queryClient.invalidateQueries({ queryKey: QueryKeyEnum.TOP_APPS });
      notifyResult(result);
    },
    onError: (error: { message: string }) => {
      toast.error(error.message);
    },
  });
};

export const useQuitAppsMutation = () => useQuitMutation(quitApps);

export const useForceQuitAppsMutation = () => useQuitMutation(forceQuitApps);
