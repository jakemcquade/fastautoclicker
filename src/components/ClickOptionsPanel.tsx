import { Pointer } from "lucide-react";

import { ClickType, MouseButton, OptionsState, Settings } from "../types";
import Segmented from "./Segmented";

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
        <div className="panel flex flex-col gap-2 w-full max-w-full">
            <div className="panel-title">
                <Pointer className="w-3.5 h-3.5" />
                Click Options
            </div>

            <div className="flex flex-col gap-1">
                <span className="text-xs text-(--text-3)">Mouse Button</span>
                <Segmented<MouseButton>
                    aria-label="Mouse button"
                    value={optionsState.mouse_button}
                    onChange={(v) => update("mouse_button", v)}
                    options={[
                        { label: "Left", value: MouseButton.Left },
                        { label: "Middle", value: MouseButton.Middle },
                        { label: "Right", value: MouseButton.Right },
                    ]}
                />
            </div>

            <div className="flex flex-col gap-1">
                <span className="text-xs text-(--text-3)">Click Type</span>
                <Segmented<ClickType>
                    aria-label="Click type"
                    value={optionsState.click_type}
                    onChange={(v) => update("click_type", v)}
                    options={[
                        { label: "Single", value: ClickType.Single },
                        { label: "Double", value: ClickType.Double },
                    ]}
                />
            </div>
        </div>
    );
}
