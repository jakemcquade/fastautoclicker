import { Pause, Play } from "lucide-react";
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
  const buttonClass = "flex h-12.5 w-full justify-center items-center gap-2";

  return (
    <div className="grid grid-cols-2 gap-2">
      <button
        type="button"
        className={cn(buttonClass, active ? "secondary" : "primary")}
        onClick={(e) => onToggle(e)}
        disabled={active || isToggling}
      >
        <span className="flex gap-1 items-center">
          <Play className="h-5 w-5" />
          Start
        </span>
        <small className="text-xs text-muted-foreground">{hotkey}</small>
      </button>

      <button
        type="button"
        className={cn(buttonClass, active ? "danger" : "disabled")}
        onClick={(e) => onToggle(e)}
        disabled={!active || isToggling}
      >
        <Pause className="h-5 w-5" />
        <span>Stop</span>
        <small className="text-xs text-muted-foreground">
          {hotkey}
        </small>
      </button>

      <button type="button" className={cn(buttonClass, hotkeyOpen ? "disabled" : "secondary")} onClick={() => setHotkeyOpen(!hotkeyOpen)}>
        Configure Hotkey
      </button>

      <button type="button" className={cn(buttonClass, "secondary")} disabled>
        Record
      </button>
    </div>
  )
}