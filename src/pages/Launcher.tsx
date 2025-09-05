import React, { useState } from "react";
import { AccountsTab } from "../components/tabs/accounts/AccountsTab";
import { GameAccount } from "../components/tabs/accounts/AccountCard";

interface GameLauncherProps {
  activeTab: string;
}

export function Launcher({ activeTab }: GameLauncherProps) {
  const [accounts, setAccounts] = useState<GameAccount[]>([]);

  const renderContent = () => {
    switch (activeTab) {
      case "accounts":
        return (
          <AccountsTab
            accounts={accounts}
            onAccountsChange={setAccounts}
          />
        );
    }
  };

  return <>{renderContent()}</>;
}