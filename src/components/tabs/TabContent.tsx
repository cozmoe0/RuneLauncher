import React from "react";

interface TabContentProps {
  tabId: string;
  title: string;
  subtitle: string;
}

export function TabContent({ tabId, title, subtitle }: TabContentProps) {
  return (
    <div className="text-center py-20">
      <h2 className="text-white mb-4">{title}</h2>
      <p className="text-slate-400">{subtitle}</p>
    </div>
  );
}