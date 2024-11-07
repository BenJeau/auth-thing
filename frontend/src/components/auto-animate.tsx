import React, { useState, useEffect } from "react";

interface Props extends React.PropsWithChildren {
  as?: keyof JSX.IntrinsicElements | React.ComponentType;
  duration?: number;
  slideIn?: boolean;
  className?: string;
}

const AutoAnimate: React.FC<Props> = ({
  as: Component = "div",
  duration = 0.2,
  slideIn = false,
  children,
  ...rest
}) => {
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    // Set visible on mount
    const timer = setTimeout(() => {
      setIsVisible(true);
    }, 0);

    return () => {
      clearTimeout(timer);
    };
  }, []);

  const style = {
    transform: `translateY(${!isVisible ? (slideIn ? "10px" : "0") : "0"})`,
    opacity: isVisible ? 1 : 0,
    transition: `all ${duration}s ease-out`,
  };

  return (
    <Component style={style} {...rest}>
      {children}
    </Component>
  );
};

export default AutoAnimate;
