import { useEffect, useState } from "react";

import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

import { ClickTypes, MouseButtonTypes } from "./types";
import "./App.css";

// function convertMil(miliseconds: number, type: "seconds" | "minutes" | "hours") { return miliseconds / (type === "seconds" ? 1000 : type === "minutes" ? 60000 : 3600000); }
export default function App() {
  const [settings, setSettings] = useState<Record<string, any> | null>(null);
  const [mouseButton, setMouseButton] = useState<MouseButtonTypes>("left");
  const [clickType, setClickType] = useState<ClickTypes>("single");
  const [recordedInput, setRecordedInput] = useState<string[]>([]);
  const [watchingInput, setWatching] = useState(false);
  const [hotkeyOpen, setHotkeyOpen] = useState(false);
  const [miliseconds, setMiliseconds] = useState(100);
  const [hotkey, setHotkey] = useState("");
  const [seconds, setSeconds] = useState(0);
  const [minutes, setMinutes] = useState(0);
  const [hours, setHours] = useState(0);
  const [time, setTime] = useState(100);

  const [active, setActive] = useState(false);
  useEffect(() => {
    async function loadStates() {
      const settings = JSON.parse(await invoke("get_settings")) as Record<string, any>;
      setSettings(settings);

      settings.mouse_button && setMouseButton(settings.mouse_button);
      settings.click_type && setClickType(settings.click_type);
      settings.hotkey && setHotkey(settings.hotkey);

      // const savedTime = await get_state<number>("interval", "U64");
      // setTime(savedTime);
      // setMiliseconds(savedTime);
      // setSeconds(convertMil(savedTime, "seconds"));
      // setMinutes(convertMil(savedTime, "minutes"));
      // setHours(convertMil(savedTime, "hours"));
    };

    loadStates();
    listen<boolean>("state", (event) => setActive(event.payload));
    listen<string>("hotkey", (event) => setHotkey(event.payload));
  }, []);

  function hotkeyCallback(value: string) { setHotkeyOpen(false); setHotkey(value); };
  // async function get_state<T>(name: string, type: "String" | "U64" = "String"): Promise<T> {
  //   let data = await invoke("get_state", { name }) ?? {} as T;
  //   if (data && typeof data === "object") data = data?.[type as keyof typeof data];

  //   return data as T;
  // };

  async function set_state(name: string, value: string | number, callback: (...args: any) => void) {
    callback(value);

    if (settings && name !== "interval") {
      let newSettings = settings;
      newSettings[name] = value;
      await invoke("set_settings", { fileContent: JSON.stringify(newSettings) });
    }

    console.log("Setting state:", name, value);
    return await invoke("set_state", { name, value: typeof value === "number" ? { U64: value } : { String: value } });
  }

  async function toggle() {
    const state = await invoke<boolean>("app_toggle", {
      mousebutton: mouseButton,
      clicktype: clickType,
      time
    });

    setActive(state);
  };

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (watchingInput !== true) return;
      if (event.repeat === true) return;

      if (recordedInput?.[0] !== event.key) {
        const payload = [...recordedInput, event.key];
        setRecordedInput(payload);

        if (payload.length === 2) setWatching(false);
      };
    };
    document.addEventListener("keydown", handleKeyDown);
  
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [watchingInput, recordedInput]);

  return (<>
    <div className={"container"}>
      <form>
        <fieldset className={"input-container"}>
          <legend>Click Interval</legend>
          <div className={"form-input"}>
            <input type={"number"} id="hours" defaultValue={0} min={0} onChange={(e) => { setHours(parseInt(e.target.value)); set_state("interval", (parseInt(e.target.value) * 60 * 60 * 1000) + (minutes * 60 * 1000) + (seconds * 1000) + miliseconds, setTime) }} />
            <label htmlFor="hours">Hours</label>
          </div>
          <div className={"form-input"}>
            <input type={"number"} id="minutes" defaultValue={0} min={0} onChange={(e) => { setMinutes(parseInt(e.target.value)); set_state("interval", (hours * 60 * 60 * 1000) + (parseInt(e.target.value) * 60 * 1000) + (seconds * 1000) + miliseconds, setTime) }} />
            <label htmlFor="minutes">Minutes</label>
          </div>
          <div className={"form-input"}>
            <input type={"number"} id="seconds" defaultValue={0} min={0} onChange={(e) => { setSeconds(parseInt(e.target.value)); set_state("interval", (hours * 60 * 60 * 1000) + (minutes * 60 * 1000) + (parseInt(e.target.value) * 1000) + miliseconds, setTime) }} />
            <label htmlFor="seconds">Seconds</label>
          </div>
          <div className={"form-input"}>
            <input type={"number"} id="miliseconds" defaultValue={100} min={1} max={1000} onChange={(e) => { setMiliseconds(parseInt(e.target.value)); set_state("interval", (hours * 60 * 60 * 1000) + (minutes * 60 * 1000) + (seconds * 1000) + parseInt(e.target.value), setTime) }} />
            <label htmlFor="miliseconds">Miliseconds</label>
          </div>
        </fieldset>
        <div style={{ display: "flex", gap: "1rem", paddingBottom: ".4rem" }}>
          <fieldset className={"input-container"} style={{ flexDirection: "column", gap: "1rem" }}>
            <legend>Click Options</legend>
            <div className={"form-select"}>
              <p>Mouse Button</p>
              <select defaultValue={mouseButton} onChange={(e) => set_state("mouse_button", e.target.value as "left" | "middle" | "right", setMouseButton)}>
                <option value={"left"} {...mouseButton === "left" && { selected: true }}>Left</option>
                <option value={"middle"} {...mouseButton === "middle" && { selected: true }}>Middle</option>
                <option value={"right"} {...mouseButton === "right" && { selected: true }}>Right</option>
              </select>
            </div>
            <div className={"form-select"}>
              <p>Click Type</p>
              <select defaultValue={clickType} onChange={(e) => set_state("click_type", e.target.value as "single" | "double", setClickType)}>
                <option value={"single"} {...clickType === "single" && { selected: true }}>Single</option>
                <option value={"double"} {...clickType === "double" && { selected: true }}>Double</option>
              </select>
            </div>
          </fieldset>
          <fieldset className={"input-container"}>
            <legend>Click Repeat</legend>
            <p style={{ height: "-webkit-fill-available", width: "-webkit-fill-available", fontFamily: "sans-serif", fontSize: "x-large", fontWeight: "600", alignContent: "center", color: "rgba(255, 255, 255, .2)" }}>COMING SOON</p>
          </fieldset>
        </div>
        <div className={"buttons-grid"}>
          <button type={"button"} onClick={toggle} disabled={active}>Start ({hotkey})</button>
          <button type={"button"} onClick={toggle} disabled={!active}>Stop ({hotkey})</button>
          <button type={"button"} disabled={hotkeyOpen !== false} onClick={async () => setHotkeyOpen(!hotkeyOpen)}>Hotkey Config</button>
          <button type={"button"} disabled={true}>Record</button>
        </div>
      </form>
      <div className={"blur"} data-state={hotkeyOpen ? "open" : "closed"} />
      <div className={"popup"} data-state={hotkeyOpen ? "open" : "closed"}>
        <div className={"content"}>
          <div className={"form-hotkey"} style={{ width: "100%", gap: "1rem" }}>
            <input type={"text"} value={recordedInput.join("+")} readOnly />
            <button type={"button"} disabled={watchingInput} onClick={() => {
              setRecordedInput([]);
              setWatching(!watchingInput);
            }}>Start</button>
          </div>
          <div className={"buttons"} style={{ paddingTop: ".5rem" }}>
            <button type={"button"} disabled={watchingInput} onClick={() => set_state("hotkey", recordedInput.join("+"), hotkeyCallback)}>Done</button>
            <button type={"button"} disabled={watchingInput} onClick={() => {
              setRecordedInput([]);
              setHotkeyOpen(false);
            }}>Cancel</button>
          </div>
        </div>
      </div>
    </div>
  </>)
}