import { Pointer } from "lucide-react";

import { OptionsState, Settings } from "../types";

export interface ClickOptionsPanelProps {
    optionsState: OptionsState;
    setOptionsState: React.Dispatch<React.SetStateAction<OptionsState>>,
    setSetting: <K extends keyof Settings>(key: K, value: Settings[K]) => void;
}

export default function ClickOptionsPanel({ setSetting, optionsState, setOptionsState }: ClickOptionsPanelProps) {
    function update<K extends keyof OptionsState & keyof Settings>(part: K, value: Settings[K]) {
        if (optionsState[part] === value) return;

        const newState = { ...optionsState, [part]: value };
        setOptionsState(newState);
        setSetting(part, value);
    }

    return (
        <div className="flex flex-col gap-1 bg-(--bg-panel) border border-(--border-subtle) rounded-xl p-2.5 w-full max-w-full">
            <div className="flex gap-1 items-center">
                <Pointer className="w-4 h-4" />
                <span className="text-sm">Click Options</span>
            </div>

            <div className="flex gap-1 justify-between items-center">
                <p className="text-sm">Mouse Button</p>
                <select value={optionsState.mouse_button} onChange={(e) => update<"mouse_button">("mouse_button", parseInt(e.target.value))}>
                    <option value="0">Left</option>
                    <option value="2">Middle</option>
                    <option value="1">Right</option>
                </select>
            </div>
            
            <div className="flex gap-1 justify-between items-center">
                <p className="text-sm">Click Type</p>
                <select value={optionsState.click_type} onChange={(e) => update<"click_type">("click_type", parseInt(e.target.value))}>
                    <option value="0">Single</option>
                    <option value="1">Double</option>
                </select>
            </div>
        </div>
    );
}
