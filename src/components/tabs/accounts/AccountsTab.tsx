import React, { useState, useEffect } from "react";
import { AnimatePresence, motion } from "motion/react";
import { LauncherHeader } from "../../LauncherHeader";
import { AccountCard, GameAccount } from "./AccountCard";
import { AddAccountDialog } from "./AddAccountDialog";
import { Character } from "./CharacterCard";
import { Button } from "../../ui/button";
import { LoadingSpinner } from "../../LoadingSpinner";
import { Plus } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface AccountsTabProps {
  accounts: GameAccount[];
  onAccountsChange: (accounts: GameAccount[]) => void;
}

const mockAccounts: GameAccount[] = [];

export function AccountsTab({ accounts: initialAccounts, onAccountsChange }: AccountsTabProps) {
  const [accounts, setAccounts] = useState<GameAccount[]>(initialAccounts.length > 0 ? initialAccounts : mockAccounts);
  const [expandedAccount, setExpandedAccount] = useState<string | null>("1");
  const [playingCharacter, setPlayingCharacter] = useState<string | null>(null);
  const [isLoggingIn, setisLoggingIn] = useState<boolean>(false);
  const [buttonText, setButtonText] = useState<string>("Add Jagex Account");

  // Update parent whenever accounts change
  React.useEffect(() => {
    onAccountsChange(accounts);
  }, [accounts, onAccountsChange]);

  // Set up event listeners for login progress updates
  useEffect(() => {
    let unlistenProgress: (() => void) | undefined;
    let unlistenComplete: (() => void) | undefined;
    let unlistenAccountAdded: (() => void) | undefined;

    const setupListeners = async () => {
      // Listen for login progress events
      [unlistenProgress, unlistenComplete, unlistenAccountAdded] = await Promise.all([listen("login-progress", (event) => {
        setButtonText(event.payload as string);
      }), listen("login-complete", () => {
        setButtonText("Add Jagex Account");
      }), listen("account-added", (event) => {
        const rustAccount = event.payload as {
          email: string;
          account_name: string;
          characters: Array<{
            account_id: string;
            display_name: string;
            user_hash: string;
            is_members: boolean;
          }>;
        };

        // Map Rust GameCharacter to frontend Character interface
        const mappedCharacters: Character[] = rustAccount.characters.map(char => ({
          id: char.account_id,
          name: char.display_name !== "null" ? char.display_name : "",
          level: 1, // Default level since isn't provided by Jagex API
          className: char.display_name !== "null" ? (char.is_members ? "Member" : "Free Player") : "New Character",
          lastPlayed: char.display_name !== "null" ? "Never" : "Never", // Not provided by API
          totalPlaytime: char.display_name !== "null" ? "Unknown" : "0h 0m" // Not provided by API
        }));

        // Create GameAccount for frontend
        const newAccount: GameAccount = {
          id: Date.now().toString(),
          accountName: rustAccount.account_name,
          email: rustAccount.email,
          characters: mappedCharacters
        };

        // Add the account
        setAccounts(prevAccounts => [...prevAccounts, newAccount]);
      })]);
      // Listen for login complete events

      // Listen for account-added events from Rust
    };

    setupListeners();

    // Cleanup listeners on unmount
    return () => {
      if (unlistenProgress) unlistenProgress();
      if (unlistenComplete) unlistenComplete();
      if (unlistenAccountAdded) unlistenAccountAdded();
    };
  }, []);

  const toggleAccount = (accountId: string) => {
    // If clicking on the currently expanded account, collapse it
    // Otherwise, expand the clicked account (and collapse any other)
    setExpandedAccount(expandedAccount === accountId ? null : accountId);
  };

  const removeAccount = (accountId: string) => {
    setAccounts(accounts.filter((account) => account.id !== accountId));
    // If we're removing the currently expanded account, clear the expansion
    if (expandedAccount === accountId) {
      setExpandedAccount(null);
    }
  };

  const addAccount = (accountData: Omit<GameAccount, "id" | "characters">) => {
    const newAccount: GameAccount = {
      id: Date.now().toString(),
      accountName: accountData.accountName,
      email: accountData.email,
      characters: [],
    }
    setAccounts([...accounts, newAccount]);
  };

  const createCharacter = (accountId: string) => {
    const newCharacter: Character = {
      id: Date.now().toString(),
      name: "", // Blank name until set in-game
      level: 1,
      className: "New Character",
      lastPlayed: "Never",
      totalPlaytime: "0h 0m",
    };

    setAccounts(
      accounts.map((account) =>
        account.id === accountId
          ? {
              ...account,
              characters: [...account.characters, newCharacter],
            }
          : account
      )
    );
  };

  const playCharacter = (characterId: string) => {
    setPlayingCharacter(characterId);
    // Simulate launching game
    setTimeout(() => {
      setPlayingCharacter(null);
    }, 2000);
  };

  const loginWithJagexAuth = async () => {
    setisLoggingIn(true);
    try {
      await invoke("login", {});
    } catch (error) {
      console.error("Login failed:", error);
    } finally {
      setisLoggingIn(false);
    }
  };

  return (
    <div className="flex flex-col h-full min-w-xl">
      <LauncherHeader />

      <div className="flex-1 overflow-y-auto space-y-5">
        <AnimatePresence>
          {accounts.map((account) => (
            <AccountCard
              key={account.id}
              account={account}
              isExpanded={expandedAccount === account.id}
              playingCharacter={playingCharacter}
              onToggle={toggleAccount}
              onRemove={removeAccount}
              onCreateCharacter={createCharacter}
              onPlayCharacter={playCharacter}
            />
          ))}
        </AnimatePresence>
      </div>

      <div className="border-t border-slate-700">
        <motion.div whileHover={{ scale: 1.02 }} whileTap={{ scale: 0.98 }}>
          <Button
            onClick={loginWithJagexAuth}
            disabled={isLoggingIn}
            className="w-full bg-emerald-600 hover:bg-emerald-700 disabled:bg-emerald-700 disabled:opacity-75 text-white py-6"
          >
            {isLoggingIn ? (
              <LoadingSpinner size="sm" className="mr-2" />
            ) : (
              <Plus className="w-5 h-5 mr-2" />
            )}
            {buttonText}
          </Button>
        </motion.div>
      </div>
    </div>
  );
}