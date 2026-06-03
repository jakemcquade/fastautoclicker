import { relaunch } from "@tauri-apps/plugin-process";
import { useMemo, useState } from "react";
import useUpdater from "../hooks/useUpdater";

export default function UpdatePopup() {
    const { update, isUpdating, setIsUpdating } = useUpdater();
    const [dismissed, setDismissed] = useState(false);

    const releasedAt = useMemo(() => {
        if (!update?.date) return "Unknown";

        const parsed = new Date(update.date);
        if (Number.isNaN(parsed.getTime())) return "Unknown";

        return parsed.toLocaleString();
    }, [update?.date]);

    if (!update || dismissed) return null;

    return (
        <div className="absolute inset-x-2 bottom-2">
            <div className="bg-[var(--bg-panel)] border border-black/50 rounded-xl p-3 text-left shadow-sm">
                <div className="flex items-start justify-between gap-3">
                    <div className="flex gap-3">
                        <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-[var(--accent-tint)]">
                            <img src="/svgs/download.svg" className="h-4 w-4" />
                        </div>
                        <div className="flex flex-col gap-1">
                            <p className="text-sm font-semibold">New update available</p>
                            <p className="text-xs text-[var(--text-2)]">
                                Version {update.version} released on {releasedAt}
                            </p>
                            <p className="text-xs text-[var(--text-2)]">Would you like to update?</p>
                        </div>
                    </div>
                    <span className="text-xs rounded-full border border-black/20 px-2 py-0.5 bg-[var(--bg-window)]">
                        v{update.version}
                    </span>
                </div>

                <div className="mt-3 flex gap-2 justify-end">
                    <button
                        type="button"
                        className="px-3 py-1.5"
                        onClick={() => setDismissed(true)}
                        disabled={isUpdating}
                    >
                        No
                    </button>
                    <button
                        type="button"
                        className="px-3 py-1.5 primary"
                        disabled={isUpdating}
                        onClick={async () => {
                            if (isUpdating) return;
                            setIsUpdating(true);

                            try {
                                await update.downloadAndInstall();
                                await relaunch();
                            } catch (err) {
                                console.error("Failed to install update!", err);
                                setIsUpdating(false);
                            }
                        }}
                    >
                        {isUpdating ? "Updating..." : "Yes, update"}
                    </button>
                </div>
            </div>
        </div>
    );
}