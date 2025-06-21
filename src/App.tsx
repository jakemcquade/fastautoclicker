import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { ClickTypes, MouseButtonTypes } from "./types";
import { buildPayout } from "./components/util";
import { HotkeyPopup } from "./components/hotkey";

type IntervalState = {
  hours: number;
  minutes: number;
  seconds: number;
  miliseconds: number;
  time: number;
};

type OptionsState = {
  mouseButton: MouseButtonTypes;
  clickType: ClickTypes;
};

export default function App() {
  const [settings, setSettings] = useState<Record<string, any> | null>(null);
  const [hotkeyOpen, setHotkeyOpen] = useState(false);
  const [active, setActive] = useState(false);
  const [hotkey, setHotkey] = useState("");
  const [intervalState, setIntervalState] = useState<IntervalState>({
    hours: 0,
    minutes: 0,
    seconds: 0,
    miliseconds: 100,
    time: 100,
  });

  const [optionsState, setOptionsState] = useState<OptionsState>({
    mouseButton: "left",
    clickType: "single",
  });

  useEffect(() => {
    async function loadStates() {
      const settings = JSON.parse(await invoke("get_settings")) as Record<string, any>;
      setSettings(settings);

      setOptionsState({
        mouseButton: settings.mouse_button ?? "left",
        clickType: settings.click_type ?? "single",
      });
      settings.hotkey && setHotkey(settings.hotkey);
    }

    loadStates();
    listen<boolean>("state", (event) => setActive(event.payload));
    listen<string>("hotkey", (event) => setHotkey(event.payload));
  }, []);

  function convertState(name: string, value: string | number) {
    const mouseButtonMap = {
      left: 0,
      right: 1,
      middle: 2
    };

    const clickTypeMap = {
      single: 0,
      double: 1
    };

    if (name === "mouse_button" && typeof value === "string") {
      return mouseButtonMap[value as keyof typeof mouseButtonMap] ?? value;
    }

    if (name === "click_type" && typeof value === "string") {
      return clickTypeMap[value as keyof typeof clickTypeMap] ?? value;
    }

    return value;
  }

  async function set_state(name: string, value: string | number, callback: (...args: any) => void) {
    callback(value);
    value = convertState(name, value);

    if (settings && name !== "interval") {
      let newSettings = { ...settings, [name]: value };
      setSettings(newSettings);

      await invoke("set_settings", { fileContent: JSON.stringify(newSettings) });
    }

    const payload = buildPayout(name, value) ?? {};
    console.log("Setting state:", name, payload);

    return await invoke("set_state", { name, value: payload });
  }

  async function toggle() {
    if (active) {
      setActive(false);
      return await invoke("app_stop");
    }

    const state = await invoke<boolean>("app_toggle", {
      mousebutton: convertState("mouse_button", optionsState.mouseButton),
      clicktype: convertState("click_type", optionsState.clickType),
      time: intervalState.time
    });

    setActive(state);
  }

  function updateInterval(part: keyof IntervalState, value: number) {
    const newState = { ...intervalState, [part]: value };
    newState.time =
      (newState.hours * 60 * 60 * 1000) +
      (newState.minutes * 60 * 1000) +
      (newState.seconds * 1000) +
      newState.miliseconds;
    setIntervalState(newState);
    set_state("interval", newState.time, (v: number) => setIntervalState(s => ({ ...s, time: v })));
  }

  return (
    <div className="flex flex-col justify-center text-center h-full p-2 m-0 relative">
      <form className="flex flex-col h-full w-full mx-auto">
        <fieldset className="flex flex-row justify-center bg-[rgba(var(--secondary),1)] border border-black/50 rounded-[6px] p-2.5 w-full gap-4">
          <legend className="text-left font-normal text-base">Click Interval</legend>
          <div className="flex flex-col h-full w-full">
            <input
              type="number"
              id="hours"
              value={intervalState.hours}
              min={0}
              className="max-w-full border border-black/50 rounded-[5px] text-center p-2 h-full"
              onChange={(e) => updateInterval("hours", parseInt(e.target.value))}
            />
            <label htmlFor="hours" className="w-full text-xs text-center">
              Hours
            </label>
          </div>
          <div className="flex flex-col h-full w-full">
            <input
              type="number"
              id="minutes"
              value={intervalState.minutes}
              min={0}
              className="max-w-full border border-black/50 rounded-[5px] text-center p-2 h-full"
              onChange={(e) => updateInterval("minutes", parseInt(e.target.value))}
            />
            <label htmlFor="minutes" className="w-full text-xs text-center">
              Minutes
            </label>
          </div>
          <div className="flex flex-col h-full w-full">
            <input
              type="number"
              id="seconds"
              value={intervalState.seconds}
              min={0}
              className="max-w-full border border-black/50 rounded-[5px] text-center p-2 h-full"
              onChange={(e) => updateInterval("seconds", parseInt(e.target.value))}
            />
            <label htmlFor="seconds" className="w-full text-xs text-center">
              Seconds
            </label>
          </div>
          <div className="flex flex-col h-full w-full">
            <input
              type="number"
              id="miliseconds"
              value={intervalState.miliseconds}
              min={1}
              max={1000}
              className="max-w-full border border-black/50 rounded-[5px] text-center p-2 h-full"
              onChange={(e) => updateInterval("miliseconds", parseInt(e.target.value))}
            />
            <label htmlFor="miliseconds" className="w-full text-xs text-center">
              Milliseconds
            </label>
          </div>
        </fieldset>
        <div className="flex gap-4 pb-2">
          <fieldset className="flex flex-col gap-4 bg-[rgba(var(--secondary),1)] border border-black/50 rounded-[6px] p-2.5 w-full max-w-full">
            <legend className="text-left font-normal text-base">Click Options</legend>
            <div className="flex gap-1 justify-between items-center">
              <p className="text-sm">Mouse Button</p>
              <select
                value={optionsState.mouseButton}
                className="h-full p-[0.1rem] rounded-[5px] border border-black/50 bg-[rgba(var(--tertiary),1)]"
                onChange={(e) =>
                  set_state("mouse_button", e.target.value as MouseButtonTypes, (v: MouseButtonTypes) =>
                    setOptionsState(s => ({ ...s, mouseButton: v }))
                  )
                }
              >
                <option value="left">Left</option>
                <option value="middle">Middle</option>
                <option value="right">Right</option>
              </select>
            </div>
            <div className="flex gap-1 justify-between items-center">
              <p className="text-sm">Click Type</p>
              <select
                value={optionsState.clickType}
                className="h-full p-[0.1rem] rounded-[5px] border border-black/50 bg-[rgba(var(--tertiary),1)]"
                onChange={(e) =>
                  set_state("click_type", e.target.value as ClickTypes, (v: ClickTypes) =>
                    setOptionsState(s => ({ ...s, clickType: v }))
                  )
                }
              >
                <option value="single">Single</option>
                <option value="double">Double</option>
              </select>
            </div>
          </fieldset>
          <fieldset className="flex flex-col bg-[rgba(var(--secondary),1)] border border-black/50 rounded-[6px] p-2.5 w-full max-w-full">
            <legend className="text-left font-normal text-base">Click Repeat</legend>
            <p className="h-full w-full font-sans text-xl font-semibold align-middle text-white/20">
              COMING SOON
            </p>
          </fieldset>
        </div>
        <div className="grid grid-cols-2 min-[400px]:grid-cols-2 gap-3 pt-1">
          <button
            type="button"
            className="px-4 py-2 h-[50px] w-full"
            onClick={toggle}
            disabled={active}
          >
            Start ({hotkey})
          </button>
          <button
            type="button"
            className="px-4 py-2 h-[50px] w-full"
            onClick={toggle}
            disabled={!active}
          >
            Stop ({hotkey})
          </button>
          <button
            type="button"
            className="px-4 py-2 h-[50px] w-full"
            disabled={hotkeyOpen !== false}
            onClick={() => setHotkeyOpen(!hotkeyOpen)}
          >
            Configure Hotkey
          </button>
          <button type="button" className="px-4 py-2 h-[50px] w-full" disabled>
            Record
          </button>
        </div>
      </form>

      <HotkeyPopup
        hotkey={hotkey}
        open={hotkeyOpen}
        onSave={hotkey => set_state("hotkey", hotkey, () => setHotkeyOpen(false))}
        onClose={() => setHotkeyOpen(false)}
      />
    </div>
  );
}
