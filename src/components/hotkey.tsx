import { useEffect, useRef, useState } from "react";

export interface HotkeyPopupProps {
    hotkey: string;
    open: boolean;
    onClose: () => void;
    onSave: (hotkey: string) => void;
}

export function HotkeyPopup({ hotkey, open, onClose, onSave }: HotkeyPopupProps) {
    const [recordedInput, setRecordedInput] = useState<string[]>([]);
    const [watchingInput, setWatching] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const inputRef = useRef<HTMLInputElement>(null);

    useEffect(() => {
        if (!open) return;

        const MODIFIERS = ["Control", "Shift", "Alt", "Meta"];
        const handleKeyDown = (event: KeyboardEvent) => {
            if (event.key === "Escape") {
                onClose();
                setWatching(false);
                setRecordedInput([]);
                setError(null);
                return;
            }

            if (!watchingInput) return;
            if (event.repeat) return;

            if (recordedInput.length === 0) {
                if (!MODIFIERS.includes(event.key)) {
                    setError("The first key must be a modifier (Ctrl, Shift, Alt, or Meta).");
                    return;
                }
                setRecordedInput([event.key]);
                return;
            }

            if (recordedInput.includes(event.key)) return;
            if (recordedInput.length === 1) {
                setRecordedInput([...recordedInput, event.key]);
                setWatching(false);
            }
        };

        document.addEventListener("keydown", handleKeyDown);
        return () => document.removeEventListener("keydown", handleKeyDown);
    }, [open, watchingInput, recordedInput, onClose]);

    useEffect(() => {
        if (open && inputRef.current) {
            inputRef.current.focus();
        }
    }, [open]);

    useEffect(() => {
        if (watchingInput) setError(null);
    }, [watchingInput]);

    return (
        <>
            {/* Blur overlay */}
            <div
                className={`fixed inset-0 z-50 bg-black/80 transition-opacity duration-300 ${open ? "opacity-100 animate-fadeIn" : "opacity-0 pointer-events-none animate-fadeOut"
                    }`}
                data-state={open ? "open" : "closed"}
                aria-hidden={!open}
            />
            {/* Popup */}
            <div
                className={`fixed top-1/2 left-1/2 z-51 -translate-x-1/2 -translate-y-1/2 w-[95vw] max-w-xs sm:max-w-sm ${open ? "opacity-100 animate-fadeIn" : "opacity-0 pointer-events-none animate-fadeOut"
                    }`}
                data-state={open ? "open" : "closed"}
                role="dialog"
                aria-modal="true"
                aria-labelledby="hotkey-popup-title"
            >
                <div className="bg-[rgba(var(--tertiary),1)] rounded-[8px] p-6 text-white shadow-lg border border-black/40">
                    <div className="flex flex-col gap-1 w-full">
                        <h2 id="hotkey-popup-title" className="text-lg font-bold mb-1">Configure Hotkey</h2>
                        <p className="text-xs text-white/70 mb-2">
                            Press <span className="font-semibold">Start</span> and then your desired key combination.<br />
                            (Max 2 keys, e.g. <kbd>Ctrl</kbd>+<kbd>K</kbd>).<br />
                            Press <kbd>Escape</kbd> to cancel.
                        </p>
                        <div className="flex flex-col gap-2 w-full">
                            <label className="text-xs text-left text-white/60">Current Hotkey:</label>
                            <div className="rounded bg-black/20 px-2 py-1 text-center text-base font-mono">{hotkey || "None"}</div>
                        </div>
                        <div className="flex flex-col gap-2 w-full">
                            <label className="text-xs text-left text-white/60">New Hotkey:</label>
                            <input
                                ref={inputRef}
                                type="text"
                                value={recordedInput.join("+")}
                                readOnly
                                className="max-w-full border border-black/50 rounded-[5px] text-center p-2 h-full bg-black/10 font-mono"
                                tabIndex={-1}
                                aria-label="New hotkey"
                            />
                        </div>
                        {error && (
                            <div className="text-red-400 text-xs text-left">{error}</div>
                        )}
                        <div className="flex flex-row gap-3 pt-2">
                            <button
                                type="button"
                                className="px-4 py-2 h-[44px] w-full rounded bg-green-600 hover:bg-green-700 transition"
                                disabled={watchingInput}
                                onClick={() => {
                                    if (!recordedInput.length) {
                                        setError("Please record a hotkey first.");
                                        return;
                                    }

                                    onSave(recordedInput.join("+"));
                                    setRecordedInput([]);
                                    setWatching(false);
                                    setError(null);
                                }}
                                aria-disabled={watchingInput}
                            >
                                Save
                            </button>
                            <button
                                type="button"
                                className="px-4 py-2 h-[44px] w-full rounded bg-gray-600 hover:bg-gray-700 transition"
                                disabled={watchingInput}
                                onClick={() => {
                                    setRecordedInput([]);
                                    setWatching(false);
                                    setError(null);
                                    onClose();
                                }}
                                aria-disabled={watchingInput}
                            >
                                Cancel
                            </button>
                        </div>
                        <button
                            type="button"
                            className="px-4 py-2 h-[44px] w-full rounded bg-blue-600 hover:bg-blue-700 transition mt-2"
                            disabled={watchingInput}
                            onClick={() => {
                                setRecordedInput([]);
                                setWatching(true);
                                setError(null);
                            }}
                            aria-disabled={watchingInput}
                        >
                            {watchingInput ? "Recording..." : "Start Recording"}
                        </button>
                    </div>
                </div>
            </div>
            {/* Animations */}
            <style>
                {`
                @keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }
                @keyframes fadeOut { from { opacity: 1; } to { opacity: 0; } }
                .animate-fadeIn { animation: fadeIn 0.2s ease-in-out; }
                .animate-fadeOut { animation: fadeOut 0.2s ease-in-out; }
                `}
            </style>
        </>
    );
}
