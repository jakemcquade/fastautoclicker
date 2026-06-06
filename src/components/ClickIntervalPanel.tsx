import { useMemo } from "react";
import { Timer } from "lucide-react";

import { partsToMs, safeParseInt } from "../lib/util";
import { IntervalState, Settings } from "../types";

export interface ClickIntervalPanelProps {
    setSetting: <K extends keyof Settings>(key: K, value: Settings[K]) => void;
    intervalState: IntervalState;
    setIntervalState: React.Dispatch<React.SetStateAction<IntervalState>>;
}

const FIELDS: { key: keyof IntervalState; label: string; min: number; max?: number }[] = [
    { key: "hours", label: "hr", min: 0 },
    { key: "minutes", label: "min", min: 0, max: 59 },
    { key: "seconds", label: "sec", min: 0, max: 59 },
    { key: "milliseconds", label: "ms", min: 0, max: 999 },
];

export default function ClickIntervalPanel({ setSetting, intervalState, setIntervalState }: ClickIntervalPanelProps) {
    const time = useMemo(() => partsToMs(intervalState), [intervalState]);

    function update(part: keyof IntervalState, value: number) {
        const newState = { ...intervalState, [part]: value };
        if (partsToMs(newState) === 0) {
            newState.milliseconds = 1;
        }

        setIntervalState(newState);
        setSetting("interval", partsToMs(newState));
    }

    return (
        <div className="panel flex flex-col gap-2 w-full">
            <div className="flex justify-between items-center">
                <div className="panel-title">
                    <Timer className="w-3.5 h-3.5" />
                    Click Interval
                </div>
                <span className="text-xs font-medium text-(--accent) tabular-nums">
                    {time.toLocaleString()} ms
                </span>
            </div>

            <div className="grid grid-cols-4 gap-2">
                {FIELDS.map(({ key, label, min, max }) => (
                    <div key={key} className="flex flex-col gap-1">
                        <input
                            type="number"
                            id={key}
                            value={intervalState[key]}
                            min={min}
                            max={max}
                            className="w-full h-9 p-1 text-center tabular-nums"
                            onChange={(e) => update(key, Math.max(min, safeParseInt(e.target.value, min)))}
                        />
                        <label htmlFor={key} className="w-full text-[0.65rem] text-center text-(--text-3)">
                            {label}
                        </label>
                    </div>
                ))}
            </div>
        </div>
    );
}
