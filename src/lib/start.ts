import { updateLocations } from "./backend/location";
import { getSettings } from "./backend/settings";
import { loadFiles } from "./backend/files";

export const load = async () => {
    await updateLocations();
    await getSettings();
    await loadFiles();
}