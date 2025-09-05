import React from "react";
import { motion, AnimatePresence } from "motion/react";
import { Card } from "../../ui/card";
import { Button } from "../../ui/button";
import { Avatar, AvatarFallback } from "../../ui/avatar";
import { Separator } from "../../ui/separator";
import {
  ChevronDown,
  Plus,
  Trash2,
  User,
} from "lucide-react";
import { CharacterCard, Character } from "./CharacterCard";

export interface GameAccount {
  id: string;
  accountName: string;
  email: string;
  characters: Character[];
}

interface AccountCardProps {
  account: GameAccount;
  isExpanded: boolean;
  playingCharacter: string | null;
  onToggle: (accountId: string) => void;
  onRemove: (accountId: string) => void;
  onCreateCharacter: (accountId: string) => void;
  onPlayCharacter: (characterId: string) => void;
}

export function AccountCard({
  account,
  isExpanded,
  playingCharacter,
  onToggle,
  onRemove,
  onCreateCharacter,
  onPlayCharacter,
}: AccountCardProps) {
  const getAccountInitials = (accountName: string) => {
    return accountName
      .split(" ")
      .map((word) => word[0])
      .join("")
      .toUpperCase()
      .slice(0, 2);
  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      transition={{ duration: 0.2 }}
    >
      <Card className="bg-slate-800/90 border-slate-700 backdrop-blur-sm">
        <div className="p-4">
          {/* Header - Always visible */}
          <div className="flex items-center">
            <div 
              className="flex items-center gap-3 cursor-pointer flex-1"
              onClick={() => onToggle(account.id)}
            >
              <Avatar className="w-10 h-10">
                <AvatarFallback className="bg-emerald-600 text-white">
                  {getAccountInitials(account.accountName)}
                </AvatarFallback>
              </Avatar>
              <div className="flex-1">
                <h3 className="text-white">{account.accountName}</h3>
                <p className="text-slate-400 text-sm">{account.email}</p>
              </div>
            </div>
            
            {/* Delete button - only visible when expanded */}
            <div className="w-8 h-8 flex items-center justify-center">
              <AnimatePresence>
                {isExpanded && (
                  <motion.div
                    initial={{ opacity: 0, scale: 0.8 }}
                    animate={{ opacity: 1, scale: 1 }}
                    exit={{ opacity: 0, scale: 0.8 }}
                    transition={{ 
                      duration: 0.2,
                      ease: "easeInOut"
                    }}
                  >
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => onRemove(account.id)}
                      className="text-red-400 hover:text-red-300 hover:bg-red-950/20 w-8 h-8 p-0"
                    >
                      <Trash2 className="w-4 h-4" />
                    </Button>
                  </motion.div>
                )}
              </AnimatePresence>
            </div>

            {/* Chevron - Always visible, clickable */}
            <div 
              className="cursor-pointer p-1"
              onClick={() => onToggle(account.id)}
            >
              <motion.div
                animate={{
                  rotate: isExpanded ? 180 : 0,
                }}
                transition={{ duration: 0.3, ease: "easeInOut" }}
              >
                <ChevronDown className="w-5 h-5 text-slate-400" />
              </motion.div>
            </div>
          </div>

          {/* Expandable Content */}
          <AnimatePresence mode="wait">
            {isExpanded && (
              <motion.div
                initial={{ 
                  height: 0, 
                  opacity: 0,
                }}
                animate={{
                  height: "auto",
                  opacity: 1,
                }}
                exit={{ 
                  height: 0, 
                  opacity: 0,
                }}
                transition={{ 
                  duration: 0.3,
                  ease: "easeInOut",
                  opacity: { duration: 0.2 }
                }}
                className="overflow-hidden"
              >
                <Separator className="my-4 bg-slate-700" />

                <motion.div 
                  className="space-y-3"
                  initial={{ y: -10 }}
                  animate={{ y: 0 }}
                  exit={{ y: -10 }}
                  transition={{ duration: 0.2, delay: 0.1 }}
                >
                  <div className="flex items-center gap-2 text-slate-300">
                    <User className="w-4 h-4" />
                    <span className="text-sm uppercase tracking-wide">
                      Characters
                    </span>
                  </div>

                  <AnimatePresence>
                    {account.characters.map((character, index) => (
                      <motion.div
                        key={character.id}
                        initial={{ opacity: 0, x: -20 }}
                        animate={{ opacity: 1, x: 0 }}
                        exit={{ opacity: 0, x: -20 }}
                        transition={{ 
                          duration: 0.2, 
                          delay: index * 0.03 + 0.1
                        }}
                      >
                        <CharacterCard
                          character={character}
                          isPlaying={playingCharacter === character.id}
                          onPlay={onPlayCharacter}
                        />
                      </motion.div>
                    ))}
                  </AnimatePresence>

                  {/* Create Character Button */}
                  <motion.div
                    initial={{ opacity: 0, y: 10 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: 10 }}
                    transition={{ 
                      duration: 0.2, 
                      delay: account.characters.length * 0.03 + 0.15
                    }}
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                  >
                    <Button
                      onClick={() => onCreateCharacter(account.id)}
                      variant="outline"
                      className="w-full border-slate-600 text-slate-300 bg-slate-600 hover:bg-slate-700 hover:text-white border-dashed py-3"
                    >
                      <Plus className="w-4 h-4 mr-2" />
                      Create New Character
                    </Button>
                  </motion.div>

                  {account.characters.length === 0 && (
                    <motion.div
                      initial={{ opacity: 0, scale: 0.9 }}
                      animate={{ opacity: 1, scale: 1 }}
                      exit={{ opacity: 0, scale: 0.9 }}
                      transition={{ duration: 0.2, delay: 0.15 }}
                      className="text-center py-4 text-slate-400"
                    >
                      <User className="w-6 h-6 mx-auto mb-1 opacity-50" />
                      <p className="text-xs">No characters yet</p>
                    </motion.div>
                  )}
                </motion.div>
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      </Card>
    </motion.div>
  );
}