import { Keyboard } from "lucide-react";

import useHotkey from "../hooks/useHotkey";
import Modal from "./Modal";

export interface HotkeyPopupProps {
    hotkey: string;
    open: boolean;
    onClose: () => void;
    onSave: (hotkey: string) => void;
}

function KeyChips({ keys }: { keys: string[] }) {
    return (
        <span className="flex items-center gap-1">
            {keys.map((key, i) => (
                <span key={`${key}-${i}`} className="flex items-center gap-1">
                    {i > 0 && <span className="text-xs text-(--text-3)">+</span>}
                    <kbd className="kbd px-2 py-1 text-sm">{key}</kbd>
                </span>
            ))}
        </span>
    );
}

export function HotkeyPopup({ hotkey, open, onClose, onSave }: HotkeyPopupProps) {
    const { recorded, watching, error, setError, start, reset } = useHotkey({ open, onClose });

    const save = () => {
        if (!recorded.length) {
            setError("Record a hotkey first.");
            return;
        }
        onSave(recorded.join("+"));
    };

    const currentKeys = hotkey ? hotkey.split("+") : [];

    return (
        <Modal open={open} onClose={onClose} onExited={reset} labelledBy="hotkey-title">
            {/* Header */}
            <div className="flex items-center gap-2.5">
                <span className="flex h-9 w-9 items-center justify-center rounded-lg bg-(--accent-tint) text-(--accent)">
                    <Keyboard className="h-4 w-4" />
                </span>
                <div className="flex flex-col">
                    <h2 id="hotkey-title" className="text-sm font-semibold leading-tight">Configure Hotkey</h2>
                    <p className="text-xs text-(--text-3)">A single key or a modifier combo</p>
                </div>
            </div>

            {/* Current hotkey */}
            <div className="mt-4 flex items-center justify-between">
                <span className="text-xs text-(--text-3)">Current</span>
                {currentKeys.length
                    ? <KeyChips keys={currentKeys} />
                    : <span className="text-xs text-(--text-3)">None</span>}
            </div>

            {/* Recording surface */}
            <button
                type="button"
                onClick={start}
                className={`mt-3 flex h-20 w-full flex-col items-center justify-center gap-1.5 rounded-xl border-2 border-dashed transition ${
                    watching
                        ? "border-(--accent) bg-(--accent-tint)"
                        : "border-(--border-strong) hover:border-(--accent)"
                }`}
            >
                {watching ? (
                    recorded.length ? (
                        <KeyChips keys={recorded} />
                    ) : (
                        <span className="flex items-center gap-2 text-sm text-(--accent)">
                            <span className="h-2 w-2 animate-pulse rounded-full bg-(--accent)" />
                            Listening… press a key
                        </span>
                    )
                ) : recorded.length ? (
                    <>
                        <KeyChips keys={recorded} />
                        <span className="text-[0.65rem] text-(--text-3)">Click to re-record</span>
                    </>
                ) : (
                    <>
                        <span className="text-sm text-(--text-2)">Click to record</span>
                        <span className="text-[0.65rem] text-(--text-3)">Esc to cancel</span>
                    </>
                )}
            </button>

            {error && <p className="mt-2 text-xs text-(--danger)">{error}</p>}

            {/* Actions */}
            <div className="mt-4 grid grid-cols-2 gap-2">
                <button type="button" className="h-10 text-sm" onClick={onClose} disabled={watching}>
                    Cancel
                </button>
                <button
                    type="button"
                    className="primary h-10 text-sm"
                    onClick={save}
                    disabled={watching || !recorded.length}
                >
                    Save
                </button>
            </div>
        </Modal>
    );
}

export default HotkeyPopup;
