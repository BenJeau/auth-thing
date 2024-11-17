import { cva, VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";
import { AutoAnimate } from "@/components";

const containerVariants = cva(
  "flex gap-8 flex-col rounded-t-2xl shadow-lg flex-1 relative",
  {
    variants: {
      color: {
        emerald: "bg-emerald-200 dark:bg-emerald-950 shadow-emerald-500/30",
        cyan: "bg-cyan-200 dark:bg-cyan-950 shadow-cyan-500/30",
        fuchsia: "bg-fuchsia-200 dark:bg-fuchsia-950 shadow-fuchsia-500/30",
      },
    },
    defaultVariants: {
      color: "emerald",
    },
  }
);

interface Props
  extends Omit<React.ButtonHTMLAttributes<HTMLDivElement>, "color">,
    VariantProps<typeof containerVariants> {
  bottomContent?: React.ReactNode;
}

const Container: React.FC<Props> = ({
  color,
  className,
  bottomContent,
  children,
}) => (
  <AutoAnimate
    slideIn
    duration={0.5}
    className={cn(containerVariants({ color, className }))}
  >
    <div className="p-5 flex gap-4 flex-col sticky top-0">{children}</div>
    {bottomContent}
  </AutoAnimate>
);

export default Container;
