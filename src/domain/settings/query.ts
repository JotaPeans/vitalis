import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";

import { QueryKeyEnum } from "@/lib/constants";

import { getAutostart, listProtectableApps, setAutostart, setProtectedApps } from "./api";
import type { ProtectableAppType, SetProtectedAppsInputType } from "./types";

export const useProtectableApps = () =>
  useQuery({
    queryKey: QueryKeyEnum.PROTECTABLE_APPS,
    queryFn: async () => {
      const { data, error } = await listProtectableApps();
      if (error) throw error;
      return data;
    },
    staleTime: 0,
    refetchOnMount: "always",
  });

export const useSetProtectedAppsMutation = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (input: SetProtectedAppsInputType) => {
      const { data, error } = await setProtectedApps(input);
      if (error) throw error;
      return data;
    },
    onSuccess: (settings) => {
      queryClient.setQueryData<ProtectableAppType[]>(QueryKeyEnum.PROTECTABLE_APPS, (apps) =>
        apps?.map((app) => ({ ...app, isProtected: settings.protectedApps.includes(app.bundleId) })),
      );
      queryClient.invalidateQueries({ queryKey: QueryKeyEnum.CLEAN_CANDIDATES });
    },
    onError: (error: { message: string }) => {
      toast.error(error.message);
    },
  });
};

export const useAutostart = () =>
  useQuery({
    queryKey: QueryKeyEnum.AUTOSTART,
    queryFn: async () => {
      const { data, error } = await getAutostart();
      if (error) throw error;
      return data;
    },
  });

export const useSetAutostartMutation = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (enabled: boolean) => {
      const { data, error } = await setAutostart(enabled);
      if (error) throw error;
      return data;
    },
    onSuccess: (enabled) => {
      queryClient.setQueryData(QueryKeyEnum.AUTOSTART, enabled);
      toast.success(enabled ? "Vitalis abrirá ao iniciar o Mac" : "Abertura automática desativada");
    },
    onError: (error: { message: string }) => {
      toast.error(error.message);
    },
  });
};
