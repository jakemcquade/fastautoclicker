import { check, Update } from "@tauri-apps/plugin-updater";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { relaunch } from "@tauri-apps/plugin-process";
import { useEffect, useState } from "react";

import "./Titlebar.css";
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

    return <div className={"titlebar"} data-tauri-drag-region>
        <div className={"container"} data-tauri-drag-region>
            <div className={"options"} data-tauri-drag-region>
                <p className={"title"} data-tauri-drag-region>FastAutoClicker</p>
                <img src={"/svgs/download.svg"} onClick={async (event) => {
                    if (update) {
                        event.currentTarget.setAttribute("disabled", "true");
                        await update.downloadAndInstall().then(relaunch);
                    }
                }} className={"download"} style={{ display: update !== null ? "block" : "none" }} />
            </div>
            <div className={"options"} data-tauri-drag-region>
                <button className={"min"} onClick={async () => await getCurrentWindow().minimize()} />
                <button className={"max"} onClick={async () => await getCurrentWindow().maximize()} />
                <button className={"exit"} onClick={async () => await getCurrentWindow().close()} />
            </div>
        </div>
    </div>
}