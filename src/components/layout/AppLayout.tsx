import React from "react";
import { AnimatedBackground } from "../AnimatedBackground";
import { Sidebar } from "../Sidebar";
import { Titlebar } from "./Titlebar";

interface AppLayoutProps {
  children: React.ReactNode;
  activeTab: string;
  onTabChange: (tabId: string) => void;
}

export function AppLayout({ children, activeTab, onTabChange }: AppLayoutProps) {
  return (
    <div className="h-screen relative">
      <AnimatedBackground />
      <Titlebar />
      <Sidebar activeTab={activeTab} onTabChange={onTabChange} />
      <div className="relative z-10 p-6 flex-1 flex flex-col h-[calc(100%-4rem)] user-select-none select-none">
        <div className="max-w-lg mx-auto flex-1 flex flex-col">
          {children}
        </div>
      </div>
    </div>
  );
}