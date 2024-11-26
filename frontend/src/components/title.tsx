import { Link, LinkProps } from "@tanstack/react-router";
import { ArrowLeft } from "lucide-react";

import { EntryCount } from "@/components";

type Props = {
  title: React.ReactNode;
  extra?: React.ReactNode;
  back?: LinkProps["to"];
} & (
  | {
      count?: number;
    }
  | { titleExtra?: React.ReactNode }
);

const Title: React.FC<Props> = ({ title, back, extra, ...props }) => (
  <div className="flex flex-wrap items-center justify-between gap-2">
    <h2 className="flex items-baseline gap-2 text-2xl font-bold">
      <div className="flex items-center gap-2">
        {back && (
          <Link to={back}>
            <ArrowLeft strokeWidth={3} />
          </Link>
        )}
        {title}
      </div>
      {"count" in props && props.count !== undefined && (
        <EntryCount count={props.count} />
      )}
      {"titleExtra" in props && props.titleExtra}
    </h2>
    {extra}
  </div>
);

export default Title;
