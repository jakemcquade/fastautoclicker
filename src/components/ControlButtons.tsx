import { Keyboard, Pause, Play, Circle } from "lucide-react";
import type { MouseEvent } from "react";
import { cn } from "../lib/util";

export interface ControlButtonsProps {
  active: boolean;
  isToggling: boolean;
  onToggle: (e?: MouseEvent<HTMLButtonElement>) => void;
  hotkey: string;
  hotkeyOpen: boolean;
  setHotkeyOpen: React.Dispatch<React.SetStateAction<boolean>>;
}

export default function ControlButtons({
  active, isToggling, onToggle, hotkey, hotkeyOpen, setHotkeyOpen
}: ControlButtonsProps) {
  const actionBtn = "flex h-11 items-center justify-center gap-2 font-semibold rounded-[10px]";

  return (
    <div className="flex flex-col gap-2">
      <div className="grid grid-cols-2 gap-2">
        <button
          type="button"
          className={cn(actionBtn, "btn-success")}
          onClick={(e) => onToggle(e)}
          disabled={active || isToggling}
        >
          <Play className="h-5 w-5" />
          <span>Start</span>
          <kbd className="kbd">{hotkey}</kbd>
        </button>

        <button
          type="button"
          className={cn(actionBtn, "danger")}
          onClick={(e) => onToggle(e)}
          disabled={!active || isToggling}
        >
          <Pause className="h-5 w-5" />
          <span>Stop</span>
          <kbd className="kbd">{hotkey}</kbd>
        </button>
      </div>

      <div className="grid grid-cols-2 gap-2">
        <button
          type="button"
          className={cn("flex h-9 items-center justify-center gap-2 text-sm", hotkeyOpen && "primary")}
          onClick={() => setHotkeyOpen(!hotkeyOpen)}
        >
          <Keyboard className="h-4 w-4" />
          <span>Hotkey</span>
          <kbd className="kbd">{hotkey}</kbd>
        </button>

        <button type="button" className="flex h-9 items-center justify-center gap-2 text-sm" disabled>
          <Circle className="h-4 w-4" />
          <span>Record</span>
          <span className="text-[0.6rem] uppercase tracking-wide text-(--text-3)">Soon</span>
        </button>
      </div>
    </div>
  )
}
