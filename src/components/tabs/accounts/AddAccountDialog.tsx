import React, { useState } from "react";
import { motion } from "motion/react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "../../ui/dialog";
import { Button } from "../../ui/button";
import { Input } from "../../ui/input";
import { Label } from "../../ui/label";
import { Plus } from "lucide-react";
import { GameAccount } from "./AccountCard";

interface AddAccountDialogProps {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  onAddAccount: (account: Omit<GameAccount, "id" | "characters">) => void;
}

export function AddAccountDialog({
  isOpen,
  onOpenChange,
  onAddAccount,
}: AddAccountDialogProps) {
  const [accountName, setAccountName] = useState("");
  const [email, setEmail] = useState("");

  const handleSubmit = () => {
    if (!accountName.trim() || !email.trim()) return;

    onAddAccount({
      accountName: accountName.trim(),
      email: email.trim(),
    });

    // Reset form
    setAccountName("");
    setEmail("");
    onOpenChange(false);
  };

  const handleCancel = () => {
    setAccountName("");
    setEmail("");
    onOpenChange(false);
  };

  return (
    <Dialog open={isOpen} onOpenChange={onOpenChange}>
      <DialogTrigger asChild>
        <motion.div whileHover={{ scale: 1.02 }} whileTap={{ scale: 0.98 }}>
          <Button className="w-full bg-emerald-600 hover:bg-emerald-700 text-white py-6">
            <Plus className="w-5 h-5 mr-2" />
            Add Jagex Account
          </Button>
        </motion.div>
      </DialogTrigger>
      <DialogContent className="bg-slate-800 border-slate-700 text-white">
        <DialogHeader>
          <DialogTitle className="text-white">Add New Game Account</DialogTitle>
          <DialogDescription className="text-slate-400">
            Enter your account details to add a new game account to the launcher.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <div className="grid gap-2">
            <Label htmlFor="account-name" className="text-slate-300">
              Account Name
            </Label>
            <Input
              id="account-name"
              value={accountName}
              onChange={(e) => setAccountName(e.target.value)}
              placeholder="Enter account name"
              className="bg-slate-700 border-slate-600 text-white placeholder:text-slate-400"
              onKeyDown={(e) => e.key === "Enter" && handleSubmit()}
            />
          </div>
          <div className="grid gap-2">
            <Label htmlFor="account-email" className="text-slate-300">
              Email
            </Label>
            <Input
              id="account-email"
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              placeholder="Enter email address"
              className="bg-slate-700 border-slate-600 text-white placeholder:text-slate-400"
              onKeyDown={(e) => e.key === "Enter" && handleSubmit()}
            />
          </div>
        </div>
        <div className="flex gap-2 justify-end">
          <Button
            variant="outline"
            onClick={handleCancel}
            className="border-slate-600 text-slate-300 hover:bg-slate-700"
          >
            Cancel
          </Button>
          <Button
            onClick={handleSubmit}
            disabled={!accountName.trim() || !email.trim()}
            className="bg-emerald-600 hover:bg-emerald-700 text-white"
          >
            Add Account
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}