import { check, Update } from "@tauri-apps/plugin-updater";
import { useEffect, useState } from "react";

export default function useUpdater() {
  const [update, setUpdate] = useState<Update | null>(null);
  const [isUpdating, setIsUpdating] = useState(false);

  useEffect(() => {
    let mounted = true;
    const checkUpdate = () =>
      check()
        .then((available) => {
          if (!mounted || !available) return;
          setUpdate(available);
        })
        .catch((err) => console.error("Failed to check for new update!", err));

    checkUpdate();
    const intervalId = setInterval(checkUpdate, 10 * 60 * 1000);

    return () => {
      mounted = false;
      clearInterval(intervalId);
    }
  }, []);

  return { update, isUpdating, setIsUpdating };
}
