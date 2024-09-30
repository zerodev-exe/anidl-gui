import { checkUpdate, installUpdate, onUpdaterEvent } from '@tauri-apps/api/updater';
import { relaunch } from '@tauri-apps/api/process';

export const checkForUpdates = async () => {
    const unlisten = await onUpdaterEvent(({ error, status }) => {
        console.log('Updater event', error, status);
    });

    try {
        const { shouldUpdate, manifest } = await checkUpdate();

        if (shouldUpdate) {
            console.log(`Installing update ${manifest?.version}`);
            await installUpdate();
            await relaunch(); // Restart the app after installation
        }
    } catch (error) {
        console.error('Error checking for updates:', error);
    } finally {
        unlisten(); // Clean up the event listener
    }
};
