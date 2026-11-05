import { ShieldCheck } from "lucide-react";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { useProtectableApps, useSetProtectedAppsMutation } from "@/domain/settings/query";
import { cn } from "@/lib/utils";

import ProtectedAppRow from "./ProtectedAppRow";

const ProtectedAppsCard = ({ className, ...props }: React.ComponentProps<"div">) => {
  const { data: apps, isPending } = useProtectableApps();
  const setProtectedApps = useSetProtectedAppsMutation();

  const toggle = (bundleId: string, isProtected: boolean) => {
    const current = (apps ?? []).filter((app) => app.isProtected).map((app) => app.bundleId);
    const bundleIds = isProtected ? [...current, bundleId] : current.filter((id) => id !== bundleId);
    setProtectedApps.mutate({ bundleIds });
  };

  return (
    <Card className={cn(className)} {...props}>
      <CardHeader>
        <CardTitle className="flex items-center gap-1.5">
          <ShieldCheck className="size-3.5" />
          Apps protegidos
        </CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-1 pt-1">
        <p className="text-muted-foreground text-[11px]">Apps protegidos nunca aparecem na limpeza de memória.</p>
        {isPending && <Skeleton className="h-32" />}
        {apps && (
          <ul className="divide-border/60 flex flex-col divide-y">
            {apps.map((app) => (
              <ProtectedAppRow
                key={app.bundleId}
                app={app}
                disabled={setProtectedApps.isPending}
                onProtectedChange={(isProtected) => toggle(app.bundleId, isProtected)}
              />
            ))}
          </ul>
        )}
      </CardContent>
    </Card>
  );
};

export default ProtectedAppsCard;
