import { updateLocations } from "./backend/location"

export const load = async () => {
    await updateLocations();
}