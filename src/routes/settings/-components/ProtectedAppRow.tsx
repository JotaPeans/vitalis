import AppIcon from "@/components/AppIcon";
import { Switch } from "@/components/ui/switch";
import { cn } from "@/lib/utils";
import type { ProtectableAppType } from "@/domain/settings/types";

interface ProtectedAppRowProps extends React.ComponentProps<"li"> {
  app: ProtectableAppType;
  disabled: boolean;
  onProtectedChange: (isProtected: boolean) => void;
}

const ProtectedAppRow = ({ app, disabled, onProtectedChange, className, ...props }: ProtectedAppRowProps) => {
  return (
    <li className={cn("flex items-center gap-2.5 py-2", className)} {...props}>
      <AppIcon icon={app.icon} name={app.name} />
      <span className="min-w-0 flex-1 truncate text-xs font-medium">{app.name}</span>
      <Switch
        checked={app.isProtected}
        disabled={disabled}
        onCheckedChange={onProtectedChange}
        aria-label={`Proteger ${app.name}`}
      />
    </li>
  );
};

export default ProtectedAppRow;
