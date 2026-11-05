import { Link } from "@tanstack/react-router";
import { Sparkles } from "lucide-react";

import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

const CleanMemoryButton = ({ className, ...props }: React.ComponentProps<"div">) => {
  return (
    <div className={cn("border-t px-4 py-3", className)} {...props}>
      <Button className="w-full" size="lg" asChild>
        <Link to="/clean">
          <Sparkles />
          Limpar memória RAM
        </Link>
      </Button>
    </div>
  );
};

export default CleanMemoryButton;
