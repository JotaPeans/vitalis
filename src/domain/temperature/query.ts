import { useQuery } from "@tanstack/react-query";

import { QueryKeyEnum } from "@/lib/constants";

import { getTemperatures } from "./api";

export const useTemperatures = () =>
  useQuery({
    queryKey: QueryKeyEnum.TEMPERATURES,
    queryFn: async () => {
      const { data, error } = await getTemperatures();
      if (error) throw error;
      return data;
    },
    staleTime: Infinity,
  });
