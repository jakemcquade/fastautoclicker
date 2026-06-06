import { useEffect, useRef, useState } from "react";
import { commands } from "../lib/commands";
import { Settings } from "../types";

export default function useSettings() {
    const [settings, setSettings] = useState<Settings | null>(null);
    const saveTimer = useRef<number | null>(null);

    useEffect(() => {
        console.log("Loading settings...");
        commands.loadSettings().then((settings) => {
            console.log(settings);
            setSettings(settings);
        }).catch(console.error);
    }, []);

    // type StateValue = { U64: number } | { U8: number } | { String: string };

    // function toStateValue<K extends keyof Settings>(key: K, value: Settings[K]): StateValue {
    //     switch (key) {
    //         case "mouse_button":
    //         case "click_type":
    //             return { U8: value as number };
    //         case "hotkey":
    //             return { String: value as string };
    //         case "time":
    //             return { U64: value as number };
    //         default:
    //             return { U64: value as number };
    //     }
    // }

    function setSetting<K extends keyof Settings>(key: K, value: Settings[K]) {
        // const stateKey = key === "time" ? "interval" : key;
        // void invoke("set_state", { name: stateKey, value: toStateValue(key, value) });

        commands.setState(key, value).then(console.log);

        setSettings(prev => {
            if (!prev) return prev;

            const next = { ...prev, [key]: value };

            if (saveTimer.current) window.clearTimeout(saveTimer.current);
            saveTimer.current = window.setTimeout(() => commands.saveSettings(next), 300);

            return next;
        });
    }

    return { settings, setSetting };
}
