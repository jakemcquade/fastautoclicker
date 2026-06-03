import { useMemo } from "react";
import { partsToMs } from "../lib/util";
import { IntervalState, Settings } from "../types";

export interface ClickIntervalPanelProps {
    setSetting: <K extends keyof Settings>(key: K, value: Settings[K]) => void;
    intervalState: IntervalState;
    setIntervalState: React.Dispatch<React.SetStateAction<IntervalState>>;
}

export default function ClickIntervalPanel({ setSetting, intervalState, setIntervalState }: ClickIntervalPanelProps) {
    const time = useMemo(() => partsToMs(intervalState), [intervalState]);

    function update(part: keyof IntervalState, value: number) {
        const newState = { ...intervalState, [part]: value };
        setIntervalState(newState);

        setSetting("interval", partsToMs(newState) ?? 0);
    }

    return (
        <div className="flex flex-col justify-center bg-[var(--bg-panel)] border border-[var(--border-subtle)] rounded-xl p-2.5 w-full gap-2">
            <div className="flex justify-between items-center">
                <span className="text-sm">Click Interval</span>
                <span className="text-xs text-[var(--accent)]">
                    {time.toLocaleString()} ms
                </span>
            </div>

            <div className="grid grid-cols-4 gap-2 w-full max-w-100 mx-auto">
                <div className="flex flex-col h-full w-full">
                    <input
                        type="number"
                        id="hours"
                        value={intervalState.hours}
                        min={0}
                        className="border border-[var(--border-subtle)] rounded-[5px] text-center p-2 h-full"
                        onChange={(e) => update("hours", parseInt(e.target.value))}
                    />
                    <label htmlFor="hours" className="w-full text-xs text-center">
                        Hours
                    </label>
                </div>
                <div className="flex flex-col h-full w-full">
                    <input
                        type="number"
                        id="minutes"
                        value={intervalState.minutes}
                        min={0}
                        max={59}
                        className="max-w-26 border border-[var(--border-subtle)] rounded-[5px] text-center p-2 h-full"
                        onChange={(e) => update("minutes", parseInt(e.target.value))}
                    />
                    <label htmlFor="minutes" className="w-full text-xs text-center">
                        Minutes
                    </label>
                </div>
                <div className="flex flex-col h-full w-full">
                    <input
                        type="number"
                        id="seconds"
                        value={intervalState.seconds}
                        min={0}
                        max={59}
                        className="max-w-26 border border-[var(--border-subtle)] rounded-[5px] text-center p-2 h-full"
                        onChange={(e) => update("seconds", parseInt(e.target.value))}
                    />
                    <label htmlFor="seconds" className="w-full text-xs text-center">
                        Seconds
                    </label>
                </div>
                <div className="flex flex-col h-full w-full">
                    <input
                        type="number"
                        id="milliseconds"
                        value={intervalState.milliseconds}
                        min={1}
                        max={999}
                        className="max-w-26 border border-[var(--border-subtle)] rounded-[5px] text-center p-2 h-full"
                        onChange={(e) => update("milliseconds", parseInt(e.target.value))}
                    />
                    <label htmlFor="milliseconds" className="w-full text-xs text-center">
                        Milliseconds
                    </label>
                </div>
            </div>
        </div>
    );
}
