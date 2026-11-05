import { useQuery } from "@tanstack/react-query";

import { QueryKeyEnum } from "@/lib/constants";

import { getDisk } from "./api";

export const useDisk = () =>
  useQuery({
    queryKey: QueryKeyEnum.DISK,
    queryFn: async () => {
      const { data, error } = await getDisk();
      if (error) throw error;
      return data;
    },
    staleTime: Infinity,
  });
