import { invoke } from "@tauri-apps/api/core";
import { useRef, useState, type MouseEvent } from "react";

import ClickIntervalPanel from "./components/ClickIntervalPanel";
import ClickOptionsPanel from "./components/ClickOptionsPanel";
import UpdatePopup from "./components/UpdatePopup";
import { DEFAULT_INTERVAL, DEFAULT_OPTIONS } from "./constants";
import useSettings from "./hooks/useSettings";
import { useTauriEvent } from "./hooks/useTauriEvent";
import { commands } from "./lib/commands";
import { HotkeyPopup } from "./lib/hotkey";
import { msToParts, partsToMs } from "./lib/util";
import type { IntervalState, OptionsState } from "./types";
import ControlButtons from "./components/ControlButtons";

export default function App() {
  const { settings, setSetting } = useSettings();
  const [hotkeyOpen, setHotkeyOpen] = useState(false);
  const [active, setActive] = useState(false);
  const [hotkey, setHotkey] = useState(settings?.hotkey || "F6");
  const [isToggling, setIsToggling] = useState(false);

  const [clickLocation, setClickLocation] = useState<{ x: number, y: number } | null>(null);
  const [intervalState, setIntervalState] = useState<IntervalState>(settings ? msToParts(settings.time) : DEFAULT_INTERVAL);
  const [optionsState, setOptionsState] = useState<OptionsState>(DEFAULT_OPTIONS);

  useTauriEvent<boolean>("state", (ev) => setActive(ev));
  useTauriEvent<string>("hotkey", (ev) => setHotkey(ev));

  const togglingRef = useRef(false);
  async function onToggle(e?: MouseEvent<HTMLButtonElement>) {
    if (e?.detail && e.detail > 1) return;
    if (togglingRef.current || isToggling) return;
    togglingRef.current = true;
    setIsToggling(true);

    try {
      if (active) {
        setActive(false);
        await commands.stop();
        return;
      }

      setActive(await commands.start());
    } finally {
      togglingRef.current = false;
      setIsToggling(false);
    }
  }

  return (
    <div className="flex flex-col justify-center text-center h-full p-2 m-0 relative">
      <form className="flex flex-col h-full w-full mx-auto gap-2">
        <ClickIntervalPanel setSetting={setSetting} intervalState={intervalState} setIntervalState={setIntervalState} />

        <div className="flex gap-2">
          <ClickOptionsPanel setSetting={setSetting} optionsState={optionsState} setOptionsState={setOptionsState} />

          <div className="flex flex-col gap-2 bg-[var(--bg-panel)] border border-[var(--border-subtle)] rounded-xl p-2.5 w-full max-w-full">
            <label className="text-left font-normal text-base">Click Repeat</label>
            <p className="h-full w-full font-sans text-xl font-semibold align-middle text-white/20">
              COMING SOON
            </p>
          </div>
        </div>

        <ControlButtons
          active={active}
          isToggling={isToggling}
          onToggle={onToggle}
          hotkey={hotkey}
          hotkeyOpen={hotkeyOpen}
          setHotkeyOpen={setHotkeyOpen}
        />
      </form>

      <UpdatePopup />
      <HotkeyPopup
        hotkey={hotkey}
        open={hotkeyOpen}
        onClose={() => setHotkeyOpen(false)}
        onSave={hotkey => {
          setSetting("hotkey", hotkey)
          setHotkeyOpen(false)
        }}
      />
    </div>
  );
}
