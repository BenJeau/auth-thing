import React, { useState } from "react";

import { CopyButton } from "@/components";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
} from "@/components/ui/tooltip";
import config from "@/lib/config";

const ApplicationVersion: React.FC = () => {
  const [showVersion, setShowVersion] = useState(true);

  return (
    <div className="inline-flex gap-1">
      <Tooltip>
        <TooltipTrigger>
          <span
            className="italic text-black/25 dark:text-white/25"
            onClick={() => {
              setShowVersion((prev) => !prev);
            }}
          >
            {showVersion ? config.version : config.commit_sha}
          </span>
        </TooltipTrigger>
        <TooltipContent>
          Click to toggle between version and commit SHA of the application.
        </TooltipContent>
      </Tooltip>
      <CopyButton
        text={showVersion ? config.version : config.commit_sha}
        className="h-auto bg-transparent hover:bg-transparent"
      />
    </div>
  );
};

export default ApplicationVersion;
