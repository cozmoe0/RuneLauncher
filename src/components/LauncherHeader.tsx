import React from "react";
import { motion } from "motion/react";

export function LauncherHeader() {
  return (
    <div className="text-center mb-8">
      <motion.h1 
        className="text-4xl text-white mb-2 tracking-wide relative"
        initial={{ opacity: 0, y: -10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.6, ease: "easeOut" }}
      >
        Rune<span className="text-emerald-400">Launcher</span>
      </motion.h1>
      <p className="text-slate-400">Custom Jagex Launcher for Any Java/Native Client binary</p>
    </div>
  );
}