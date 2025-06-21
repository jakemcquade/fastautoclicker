import { check, Update } from "@tauri-apps/plugin-updater";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { relaunch } from "@tauri-apps/plugin-process";
import { useEffect, useState } from "react";

export default function Toolbar() {
    const [update, setUpdate] = useState<Update | null>(null);
    function checkUpdate() {
        return check().then(async (update) => {
            if (!update) return console.log("No update available.");

            console.log("Update available: ", update);
            setUpdate(update);
        }).catch((err) => console.error("Failed to check for new update!", err));
    }

    useEffect(() => {
        checkUpdate();
        setInterval(checkUpdate, (10 * 60) * 1000);
    }, []);

    return (
        <div className="w-full h-[35px] bg-[rgba(var(--secondary),0.5)]" data-tauri-drag-region>
            <div className="flex flex-row items-center justify-between px-2 py-1" data-tauri-drag-region>
                <div className="flex items-center justify-between h-full w-full gap-1 px-2" data-tauri-drag-region>
                    <p className="font-medium" data-tauri-drag-region>
                        FastAutoClicker
                    </p>
                    <img
                        src="/svgs/download.svg"
                        onClick={async (event) => {
                            if (update) {
                                event.currentTarget.setAttribute("disabled", "true");
                                await update.downloadAndInstall().then(relaunch);
                            }
                        }}
                        className="block h-[18px] w-[18px] align-top cursor-pointer text-[#00a900]"
                        style={{ display: update !== null ? "block" : "none" }}
                    />
                </div>
                <div className="flex items-center justify-between h-full gap-1 px-2" data-tauri-drag-region>
                    <button className="rounded-full block outline-none h-[18px] w-[18px] bg-[#22ff00] p-0 hover:border-0" onClick={async () => await getCurrentWindow().minimize()} />
                    <button className="rounded-full block outline-none h-[18px] w-[18px] bg-[#ffc400] p-0 hover:border-0" onClick={async () => await getCurrentWindow().maximize()} />
                    <button className="rounded-full block outline-none h-[18px] w-[18px] bg-[#ff0000] p-0 hover:border-0" onClick={async () => await getCurrentWindow().close()} />
                </div>
            </div>
        </div>
    );
}
