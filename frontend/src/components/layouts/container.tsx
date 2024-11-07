import { cva, VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";
import { AutoAnimate } from "@/components";

const containerVariants = cva(
  "flex gap-8 flex-col rounded-t-2xl shadow-lg border-2 border-b-0 flex-1 relative",
  {
    variants: {
      color: {
        emerald:
          "bg-emerald-100 dark:bg-emerald-950 border-emerald-400 dark:border-emerald-900",
        cyan: "bg-cyan-100 dark:bg-cyan-950 border-cyan-400 dark:border-cyan-900",
        fuchsia:
          "bg-fuchsia-100 dark:bg-fuchsia-950 border-fuchsia-400 dark:border-fuchsia-900",
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
