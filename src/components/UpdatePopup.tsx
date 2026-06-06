import { relaunch } from "@tauri-apps/plugin-process";
import { Download, Loader2 } from "lucide-react";
import { useMemo, useState } from "react";

import useUpdater from "../hooks/useUpdater";
import Modal from "./Modal";

export default function UpdatePopup() {
    const { update, isUpdating, setIsUpdating } = useUpdater();
    const [dismissed, setDismissed] = useState(false);

    const releasedAt = useMemo(() => {
        if (!update?.date) return "Unknown";

        const parsed = new Date(update.date);
        if (Number.isNaN(parsed.getTime())) return "Unknown";

        return parsed.toLocaleDateString(undefined, { year: "numeric", month: "short", day: "numeric" });
    }, [update?.date]);

    if (!update) return null;

    const close = () => {
        if (!isUpdating) setDismissed(true);
    };

    const install = async () => {
        if (isUpdating) return;
        setIsUpdating(true);

        try {
            await update.downloadAndInstall();
            await relaunch();
        } catch (err) {
            console.error("Failed to install update!", err);
            setIsUpdating(false);
        }
    };

    return (
        <Modal open={!dismissed} onClose={close} labelledBy="update-title">
            {/* Header */}
            <div className="flex items-center gap-2.5">
                <span className="flex h-9 w-9 items-center justify-center rounded-lg bg-(--accent-tint) text-(--accent)">
                    <Download className="h-4 w-4" />
                </span>
                <div className="flex flex-1 flex-col">
                    <h2 id="update-title" className="text-sm font-semibold leading-tight">Update available</h2>
                    <p className="text-xs text-(--text-3)">A new version is ready to install</p>
                </div>
                <span className="kbd">v{update.version}</span>
            </div>

            {/* Details */}
            <div className="mt-4 flex items-center justify-between rounded-xl border border-(--border-subtle) bg-(--bg-panel) px-3 py-2">
                <span className="text-xs text-(--text-3)">Released</span>
                <span className="text-xs text-(--text-2)">{releasedAt}</span>
            </div>

            {/* Actions */}
            <div className="mt-4 grid grid-cols-2 gap-2">
                <button type="button" className="h-10 text-sm" onClick={close} disabled={isUpdating}>
                    Later
                </button>
                <button
                    type="button"
                    className="primary flex h-10 items-center justify-center gap-2 text-sm"
                    onClick={install}
                    disabled={isUpdating}
                >
                    {isUpdating
                        ? <><Loader2 className="h-4 w-4 animate-spin" /> Updating…</>
                        : <><Download className="h-4 w-4" /> Update now</>}
                </button>
            </div>
        </Modal>
    );
}
