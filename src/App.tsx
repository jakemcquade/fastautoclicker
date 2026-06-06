import { useEffect, useRef, useState, type MouseEvent } from "react";

import { DEFAULT_INTERVAL, DEFAULT_OPTIONS, DEFAULT_REPEAT } from "./constants";
import type { IntervalState, OptionsState, RepeatState } from "./types";
import { useTauriEvent } from "./hooks/useTauriEvent";
import useSettings from "./hooks/useSettings";
import { commands } from "./lib/commands";
import { msToParts } from "./lib/util";

import ClickIntervalPanel from "./components/ClickIntervalPanel";
import ClickOptionsPanel from "./components/ClickOptionsPanel";
import ClickRepeatPanel from "./components/ClickRepeatPanel";
import ControlButtons from "./components/ControlButtons";
import { HotkeyPopup } from "./components/HotkeyPopup";
import UpdatePopup from "./components/UpdatePopup";

export default function App() {
  const { settings, setSetting } = useSettings();
  const [hotkeyOpen, setHotkeyOpen] = useState(false);
  const [active, setActive] = useState(false);
  const [hotkey, setHotkey] = useState(settings?.hotkey || "F6");
  const [isToggling, setIsToggling] = useState(false);

  const [intervalState, setIntervalState] = useState<IntervalState>(settings?.interval ? msToParts(settings.interval) : DEFAULT_INTERVAL);
  const [optionsState, setOptionsState] = useState<OptionsState>(DEFAULT_OPTIONS);
  const [repeatState, setRepeatState] = useState<RepeatState>(DEFAULT_REPEAT);

  const repeatHydrated = useRef(false);
  useEffect(() => {
    if (!settings || repeatHydrated.current) return;
    repeatHydrated.current = true;
  
    const stored = settings.repeat ?? 0;
    setRepeatState(stored > 0
      ? { mode: "count", count: stored }
      : { mode: "until_stopped", count: DEFAULT_REPEAT.count });
  }, [settings]);

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

      setActive(true);
      await commands.start();
    } finally {
      togglingRef.current = false;
      setIsToggling(false);
    }
  }

  return (
    <div className="flex flex-col h-full p-2 m-0 gap-2 relative">
      <ClickIntervalPanel setSetting={setSetting} intervalState={intervalState} setIntervalState={setIntervalState} />

      <div className="grid grid-cols-2 gap-2">
        <ClickOptionsPanel setSetting={setSetting} optionsState={optionsState} setOptionsState={setOptionsState} />
        <ClickRepeatPanel repeatState={repeatState} setRepeatState={setRepeatState} setSetting={setSetting} />
      </div>

      <div className="mt-auto">
        <ControlButtons
          active={active}
          isToggling={isToggling}
          onToggle={onToggle}
          hotkey={hotkey}
          hotkeyOpen={hotkeyOpen}
          setHotkeyOpen={setHotkeyOpen}
        />
      </div>

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
