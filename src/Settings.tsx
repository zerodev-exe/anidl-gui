import { checkForUpdates } from '../components/utils/Update';

function Settings() {
    return <div className='container'>
        <button onClick={checkForUpdates}>Check for Updates</button>
    </div>;
}

export default Settings;
