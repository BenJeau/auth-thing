import { useState, useEffect } from "react";
import dayjs from "dayjs";

export const useResendTimer = (createdAt: string | undefined) => {
  const [canResend, setCanResend] = useState(false);
  const [timeLeft, setTimeLeft] = useState(0);

  useEffect(() => {
    if (!createdAt) return;

    const createdAtDate = dayjs.utc(createdAt);

    const calculateTimeLeft = () => {
      const diffInSeconds = 60 - dayjs.utc().diff(createdAtDate, "seconds");
      return Math.max(0, diffInSeconds);
    };

    const initial = calculateTimeLeft();
    setTimeLeft(initial);
    setCanResend(initial === 0);

    if (initial > 0) {
      const timer = setInterval(() => {
        const remaining = calculateTimeLeft();
        setTimeLeft(remaining);
        if (remaining === 0) {
          setCanResend(true);
          clearInterval(timer);
        }
      }, 1000);

      return () => clearInterval(timer);
    }
  }, [createdAt]);

  return { canResend, timeLeft, setCanResend };
};
