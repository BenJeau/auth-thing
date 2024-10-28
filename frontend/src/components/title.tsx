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
  <div className="flex justify-between items-center gap-2 flex-wrap">
    <h2 className="text-2xl font-bold flex gap-2 items-baseline">
      <div className="flex gap-2 items-center">
        {back && (
          <Link to={back}>
            <ArrowLeft strokeWidth={3} />
          </Link>
        )}
        {title}
      </div>
      {"count" in props && props.count && <EntryCount count={props.count} />}
      {"titleExtra" in props && props.titleExtra}
    </h2>
    {extra}
  </div>
);

export default Title;
