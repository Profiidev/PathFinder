import { updateLocations } from "./backend/location";
import { getSettings } from "./backend/settings";
import { loadFiles } from "./backend/files";
import { listen } from "@tauri-apps/api/event";

export const load = async () => {
    await updateLocations();
    await getSettings(false);
    await loadFiles();
    await eventListener();
}

const eventListener = async () => {
    await listen("update-locations", async (e) => {
        await updateLocations();
    });
    await listen("update-settings", async (e) => {
        await getSettings(true);
    });
}