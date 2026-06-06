import { Repeat } from "lucide-react";

import { safeParseInt } from "../lib/util";
import { RepeatMode, RepeatState, Settings } from "../types";
import Segmented from "./Segmented";

export interface ClickRepeatPanelProps {
    repeatState: RepeatState;
    setRepeatState: React.Dispatch<React.SetStateAction<RepeatState>>;
    setSetting: <K extends keyof Settings>(key: K, value: Settings[K]) => void;
}

function effectiveRepeat(state: RepeatState): number {
    return state.mode === "count" ? Math.max(1, state.count) : 0;
}

export default function ClickRepeatPanel({ repeatState, setRepeatState, setSetting }: ClickRepeatPanelProps) {
    function update<K extends keyof RepeatState>(part: K, value: RepeatState[K]) {
        if (repeatState[part] === value) return;
        const next = { ...repeatState, [part]: value };
        setRepeatState(next);
        setSetting("repeat", effectiveRepeat(next));
    }

    const countMode = repeatState.mode === "count";

    return (
        <div className="panel flex flex-col gap-2 w-full max-w-full">
            <div className="panel-title">
                <Repeat className="w-3.5 h-3.5" />
                Click Repeat
            </div>

            <div className="flex flex-col gap-1">
                <span className="text-xs text-(--text-3)">Mode</span>
                <Segmented<RepeatMode>
                    aria-label="Repeat mode"
                    value={repeatState.mode}
                    onChange={(v) => update("mode", v)}
                    options={[
                        { label: "Forever", value: "until_stopped" },
                        { label: "N Times", value: "count" },
                    ]}
                />
            </div>

            <div className="flex items-center gap-2">
                <input
                    type="number"
                    id="repeat-count"
                    min={1}
                    value={repeatState.count}
                    disabled={!countMode}
                    aria-disabled={!countMode}
                    aria-label="Repeat count"
                    className="w-full h-8 p-1 text-center tabular-nums flex-1 disabled:opacity-40"
                    onChange={(e) => update("count", Math.max(1, safeParseInt(e.target.value, 1)))}
                />
                <span className={`text-xs ${countMode ? "text-(--text-2)" : "text-(--text-3)"}`}>times</span>
            </div>
        </div>
    );
}
