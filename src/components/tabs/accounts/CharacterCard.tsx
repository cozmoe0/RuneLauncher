import React from "react";
import { motion } from "motion/react";
import { Button } from "../../ui/button";
import { Badge } from "../../ui/badge";
import { Play } from "lucide-react";
import { LoadingSpinner } from "../../LoadingSpinner";

export interface Character {
  id: string;
  name: string;
  level: number;
  className: string;
  lastPlayed: string;
  totalPlaytime: string;
}

interface CharacterCardProps {
  character: Character;
  isPlaying: boolean;
  onPlay: (characterId: string) => void;
}

export function CharacterCard({ character, isPlaying, onPlay }: CharacterCardProps) {
  return (
    <motion.div
      className="bg-slate-750 rounded-lg p-3 border border-slate-700"
      whileHover={{
        backgroundColor: "rgb(51 65 85)",
      }}
      transition={{
        duration: 0.2,
      }}
    >
      <div className="flex items-center justify-between">
        <div className="flex-1">
          <div className="flex items-center gap-2 mb-1">
            <h4
              className={`${character.name ? "text-white" : "text-slate-400 italic"}`}
            >
              {character.name || "Unnamed Character"}
            </h4>
            <Badge
              variant="secondary"
              className="bg-slate-600 text-slate-200 text-xs"
            >
              Lvl {character.level}
            </Badge>
            <Badge
              variant="outline"
              className={`border-slate-600 text-xs ${
                character.className === "New Character"
                  ? "text-amber-400 border-amber-400"
                  : "text-slate-300"
              }`}
            >
              {character.className}
            </Badge>
          </div>
          <div className="flex gap-4 text-sm text-slate-400">
            <span>Last played: {character.lastPlayed}</span>
            <span>Playtime: {character.totalPlaytime}</span>
          </div>
        </div>
        <Button
          onClick={() => onPlay(character.id)}
          disabled={isPlaying}
          className="bg-emerald-600 hover:bg-emerald-700 text-white px-6"
        >
          {isPlaying ? (
            <LoadingSpinner />
          ) : (
            <>
              <Play className="w-4 h-4 mr-2" />
              {character.name ? "Play" : "Create & Play"}
            </>
          )}
        </Button>
      </div>
    </motion.div>
  );
}