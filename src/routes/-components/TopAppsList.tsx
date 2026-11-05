import { LayoutGrid } from "lucide-react";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { useTopApps } from "@/domain/processes/query";
import { cn } from "@/lib/utils";

import TopAppRow from "./TopAppRow";

const TopAppsList = ({ className, ...props }: React.ComponentProps<"div">) => {
  const { data: apps, isPending } = useTopApps();
  const maxBytes = apps?.[0]?.footprintBytes ?? 0;

  return (
    <Card className={cn(className)} {...props}>
      <CardHeader>
        <CardTitle className="flex items-center gap-1.5">
          <LayoutGrid className="size-3.5" />
          Apps que mais usam memória
        </CardTitle>
      </CardHeader>
      <CardContent className="pt-1">
        {isPending && (
          <div className="flex flex-col gap-2">
            {Array.from({ length: 5 }, (_, i) => (
              <Skeleton key={i} className="h-7" />
            ))}
          </div>
        )}
        {apps && (
          <ul className="divide-border/60 flex flex-col divide-y">
            {apps.map((app) => (
              <TopAppRow key={app.pid} app={app} maxBytes={maxBytes} />
            ))}
          </ul>
        )}
      </CardContent>
    </Card>
  );
};

export default TopAppsList;
