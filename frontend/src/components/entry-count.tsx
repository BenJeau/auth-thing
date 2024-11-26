import { cn } from "@/lib/utils";
import { Trans } from "@/components";

interface Props {
  count: number;
  className?: string;
}

const EntryCount: React.FC<Props> = ({ count, className }) => (
  <span
    className={cn(
      "whitespace-nowrap font-normal text-sm lowercase opacity-50",
      className,
    )}
  >
    {count.toLocaleString()} <Trans id={count > 1 ? "entries" : "entry"} />
  </span>
);

export default EntryCount;
