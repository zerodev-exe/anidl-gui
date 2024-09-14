import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

function Settings() {
    const [isDub, setIsDub] = useState(false);
    const [isSub, setIsSub] = useState(false); // New state for subbed filter

    useEffect(() => {
        const fetchFilters = async () => {
            const dubFilter = await invoke("get_filter_dub") as boolean;
            const subFilter = await invoke("get_filter_sub") as boolean; // Fetch sub filter
            setIsDub(dubFilter);
            setIsSub(subFilter); // Set sub filter state
        };
        fetchFilters();
    }, []);

    const handleSaveFilters = async () => {
        await invoke('set_filter_dub', { isDub });
        await invoke('set_filter_sub', { isSub }); // Save sub filter
    };

    const handleCheckboxChangeDub = async (e: React.ChangeEvent<HTMLInputElement>) => {
        const isChecked = e.target.checked;
        setIsDub(isChecked);
        await handleSaveFilters();
    };

    const handleCheckboxChangeSub = async (e: React.ChangeEvent<HTMLInputElement>) => {
        const isChecked = e.target.checked;
        setIsSub(isChecked);
        await handleSaveFilters();
    };

    return (
        <div>
            <h1>Settings</h1>
            <label>
                <input
                    type="checkbox"
                    checked={isDub}
                    onChange={handleCheckboxChangeDub}
                />
                Only show Dubbed
            </label>
            <label>
                <input
                    type="checkbox"
                    checked={isSub}
                    onChange={handleCheckboxChangeSub}
                />
                Only show Subbed
            </label>
        </div>
    );
}

export default Settings;