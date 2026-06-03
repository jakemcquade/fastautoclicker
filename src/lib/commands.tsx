import { invoke } from "@tauri-apps/api/core";
import type { Settings } from "../types";

export const commands = {
  start: () => invoke<boolean>("app_toggle"),
  stop: () => invoke<void>("app_stop"),

  setState: (name: string, value: number | string | boolean) => {
    if (typeof value === "boolean") {
      return invoke("set_state", { name, value: { Bool: value } });
    }

    if (typeof value === "number") {
      return invoke("set_state", {
        name,
        value: name === "interval" ? { U64: value } : { U8: value },
      });
    }

    return invoke("set_state", { name, value: { String: value } });
  },
  loadSettings: () => invoke<string>("get_settings").then(s => JSON.parse(s)) as Promise<Settings>,
  saveSettings: (s: object) => invoke("set_settings", { fileContent: JSON.stringify(s) }),
};