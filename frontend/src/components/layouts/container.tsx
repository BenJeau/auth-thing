import { cva, VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";
import { AutoAnimate } from "@/components";

const containerVariants = cva(
  "flex gap-8 flex-col rounded-t-3xl backdrop-blur shadow-lg flex-1 relative",
  {
    variants: {
      color: {
        emerald:
          "bg-emerald-200/80 dark:bg-emerald-950/80 shadow-emerald-500/20",
        cyan: "bg-cyan-200/80 dark:bg-cyan-950/80 shadow-cyan-500/20",
        fuchsia:
          "bg-fuchsia-200/80 dark:bg-fuchsia-950/80 shadow-fuchsia-500/20",
      },
    },
    defaultVariants: {
      color: "emerald",
    },
  },
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
    <div className="p-6 flex gap-4 flex-col sticky top-0">{children}</div>
    {bottomContent}
  </AutoAnimate>
);

export default Container;
