import { useQuery } from "@tanstack/react-query";

import { QueryKeyEnum } from "@/lib/constants";

import { listTopApps } from "./api";

export const useTopApps = () =>
  useQuery({
    queryKey: QueryKeyEnum.TOP_APPS,
    queryFn: async () => {
      const { data, error } = await listTopApps();
      if (error) throw error;
      return data;
    },
    staleTime: Infinity,
  });
