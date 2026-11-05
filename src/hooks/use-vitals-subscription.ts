import { useEffect } from "react";
import { useQueryClient } from "@tanstack/react-query";

import { subscribeVitals } from "@/domain/vitals/api";
import { QueryKeyEnum } from "@/lib/constants";

export function useVitalsSubscription() {
  const queryClient = useQueryClient();

  useEffect(() => {
    const unlisten = subscribeVitals((vitals) => {
      queryClient.setQueryData(QueryKeyEnum.TEMPERATURES, vitals.temperatures);
      queryClient.setQueryData(QueryKeyEnum.TOP_APPS, vitals.topApps);
      if (vitals.memory) queryClient.setQueryData(QueryKeyEnum.MEMORY, vitals.memory);
      if (vitals.disk) queryClient.setQueryData(QueryKeyEnum.DISK, vitals.disk);
    });
    return () => {
      unlisten.then((stop) => stop());
    };
  }, [queryClient]);
}
