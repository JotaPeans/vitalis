import { cn, formatBytes } from "@/lib/utils";
import type { QuitOutcomeType } from "@/domain/cleaner/types";

interface QuitOutcomeListProps extends React.ComponentProps<"div"> {
  title: string;
  outcomes: QuitOutcomeType[];
  classNames?: {
    title?: string;
  };
}

const QuitOutcomeList = ({ title, outcomes, className, classNames, ...props }: QuitOutcomeListProps) => {
  return (
    <div className={cn("flex flex-col gap-1", className)} {...props}>
      <span className={cn("text-muted-foreground text-[11px] font-semibold tracking-wide uppercase", classNames?.title)}>
        {title}
      </span>
      <ul className="flex flex-col">
        {outcomes.map((outcome) => (
          <li key={outcome.pid} className="flex justify-between py-1 text-xs">
            <span className="truncate">{outcome.name}</span>
            <span className="tabular text-muted-foreground">{formatBytes(outcome.footprintBytes)}</span>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default QuitOutcomeList;
