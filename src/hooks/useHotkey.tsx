import { useEffect, useState } from "react";
import { commands } from "../lib/commands";

const MODIFIERS = ["Control", "Shift", "Alt", "Meta"];

// Physical-code → canonical key name, mirroring the backend keylistener so a
// recorded hotkey always matches what the global listener will actually detect.
const NAMED_CODES: Record<string, string> = {
    Tab: "Tab", Enter: "Enter", Escape: "Escape", Backspace: "Backspace",
    CapsLock: "CapsLock", Space: "Space", Delete: "Delete", Insert: "Insert",
    Home: "Home", End: "End", PageUp: "PageUp", PageDown: "PageDown",
    ArrowUp: "ArrowUp", ArrowDown: "ArrowDown", ArrowLeft: "ArrowLeft", ArrowRight: "ArrowRight",
};

// Resolve a key event to a canonical name. The trigger key is read from
// event.code so it stays correct regardless of held modifiers — event.key can
// report "Unidentified" for combos like Shift+Tab. Returns null for keys the
// backend can't detect, so they simply aren't recordable.
function canonicalKey(event: KeyboardEvent): string | null {
    switch (event.key) {
        case "Control": return "Control";
        case "Shift": return "Shift";
        case "Alt":
        case "AltGraph": return "Alt";
        case "Meta": return "Meta";
    }

    const code = event.code;
    if (/^Key[A-Z]$/.test(code)) return code.slice(3).toLowerCase(); // KeyA -> a
    if (/^Digit[0-9]$/.test(code)) return code.slice(5);            // Digit1 -> 1
    if (/^F([1-9]|1[0-2])$/.test(code)) return code;                // F1..F12
    return NAMED_CODES[code] ?? null;
}

const byModifierOrder = (a: string, b: string) => MODIFIERS.indexOf(a) - MODIFIERS.indexOf(b);

export interface UseHotkeyOptions {
    open: boolean;
    onClose: () => void;
}

/**
 * Captures a hotkey while the popup is open: a single key (e.g. F6) or a
 * modifier combo (e.g. Control+K). Also tells the backend when we're actively
 * recording so the global listener doesn't fire the current hotkey.
 */
export function useHotkey({ open, onClose }: UseHotkeyOptions) {
    const [recorded, setRecorded] = useState<string[]>([]);
    const [watching, setWatching] = useState(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        if (!open) return;

        const handleKeyDown = (event: KeyboardEvent) => {
            // Not recording: Escape closes the popup.
            if (!watching) {
                if (event.key === "Escape") {
                    onClose();
                    setRecorded([]);
                    setError(null);
                }
                return;
            }

            if (event.repeat) return;
            event.preventDefault();

            const key = canonicalKey(event);
            if (!key) return;

            // Escape cancels the in-progress recording.
            if (key === "Escape") {
                setWatching(false);
                setRecorded([]);
                setError(null);
                return;
            }

            // Modifiers accumulate (any number), kept in canonical order, until
            // a non-modifier trigger key completes the combo.
            if (MODIFIERS.includes(key)) {
                setRecorded(prev =>
                    prev.includes(key) ? prev : [...prev, key].sort(byModifierOrder)
                );
                return;
            }

            // Non-modifier = trigger key. Ignore if a trigger was already
            // captured, otherwise append it after the modifiers and finish.
            setRecorded(prev =>
                prev.some(k => !MODIFIERS.includes(k)) ? prev : [...prev, key]
            );
            setWatching(false);
        };

        document.addEventListener("keydown", handleKeyDown);
        return () => document.removeEventListener("keydown", handleKeyDown);
    }, [open, watching, onClose]);

    // Clear any error as soon as a new recording begins.
    useEffect(() => {
        if (watching) setError(null);
    }, [watching]);

    // Suppress the global hotkey while the popup is open / recording.
    useEffect(() => {
        void commands.setState("hotkey_recording", open ? watching : false);
    }, [open, watching]);

    function start() {
        setRecorded([]);
        setWatching(true);
        setError(null);
    }

    function reset() {
        setRecorded([]);
        setWatching(false);
        setError(null);
    }

    return { recorded, watching, error, setError, start, reset };
}

export default useHotkey;
