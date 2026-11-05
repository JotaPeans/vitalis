import { useQuery } from "@tanstack/react-query";

import { QueryKeyEnum } from "@/lib/constants";

import { getMemory } from "./api";

export const useMemory = () =>
  useQuery({
    queryKey: QueryKeyEnum.MEMORY,
    queryFn: async () => {
      const { data, error } = await getMemory();
      if (error) throw error;
      return data;
    },
    staleTime: Infinity,
  });
