import type { ReactNode } from "react";
import { Link } from "@tanstack/react-router";
import { ChevronLeft } from "lucide-react";

import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

interface PageHeaderProps extends React.ComponentProps<"header"> {
  title: string;
  subtitle?: string;
  backTo?: "/";
  actions?: ReactNode;
}

const PageHeader = ({ title, subtitle, backTo, actions, className, ...props }: PageHeaderProps) => {
  return (
    <header className={cn("flex items-center justify-between gap-2 px-4 pt-3 pb-2", className)} {...props}>
      <div className="flex min-w-0 items-center gap-1">
        {backTo && (
          <Button variant="ghost" size="icon" className="-ml-2" asChild>
            <Link to={backTo} aria-label="Voltar">
              <ChevronLeft />
            </Link>
          </Button>
        )}
        <div className="min-w-0">
          <h1 className="truncate text-[15px] font-semibold tracking-tight">{title}</h1>
          {subtitle && <p className="text-muted-foreground truncate text-[11px]">{subtitle}</p>}
        </div>
      </div>
      {actions && <div className="flex items-center gap-1">{actions}</div>}
    </header>
  );
};

export default PageHeader;
