import { AppWindow } from "lucide-react";

import { cn, toIconSrc } from "@/lib/utils";

interface AppIconProps extends React.ComponentProps<"div"> {
  icon: string | null;
  name: string;
}

const AppIcon = ({ icon, name, className, ...props }: AppIconProps) => {
  const src = toIconSrc(icon);
  return (
    <div className={cn("flex size-6 shrink-0 items-center justify-center", className)} {...props}>
      {src ? (
        <img src={src} alt={name} className="size-full object-contain" draggable={false} />
      ) : (
        <AppWindow className="text-muted-foreground size-4" />
      )}
    </div>
  );
};

export default AppIcon;
