import { Card, CardContent } from "@/components/ui/card";
import { Switch } from "@/components/ui/switch";
import { useAutostart, useSetAutostartMutation } from "@/domain/settings/query";
import { cn } from "@/lib/utils";

const AutostartCard = ({ className, ...props }: React.ComponentProps<"div">) => {
  const { data: enabled } = useAutostart();
  const setAutostart = useSetAutostartMutation();

  return (
    <Card className={cn(className)} {...props}>
      <CardContent className="flex items-center justify-between gap-3">
        <div className="flex flex-col">
          <span className="text-xs font-medium">Abrir ao iniciar o Mac</span>
          <span className="text-muted-foreground text-[11px]">O Vitalis fica sempre na barra de menus</span>
        </div>
        <Switch
          checked={enabled ?? false}
          disabled={enabled === undefined || setAutostart.isPending}
          onCheckedChange={(value) => setAutostart.mutate(value)}
          aria-label="Abrir ao iniciar o Mac"
        />
      </CardContent>
    </Card>
  );
};

export default AutostartCard;
