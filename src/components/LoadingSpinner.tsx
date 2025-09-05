import React from "react";
import { motion } from "motion/react";

interface LoadingSpinnerProps {
  size?: "sm" | "md" | "lg";
  className?: string;
}

export function LoadingSpinner({ size = "md", className = "" }: LoadingSpinnerProps) {
  const sizeClasses = {
    sm: "w-3 h-3 border",
    md: "w-4 h-4 border-2",
    lg: "w-6 h-6 border-2"
  };

  return (
    <motion.div
      animate={{ rotate: 360 }}
      transition={{
        duration: 1,
        repeat: Infinity,
        ease: "linear",
      }}
      className={`${sizeClasses[size]} border-white border-t-transparent rounded-full ${className}`}
    />
  );
}