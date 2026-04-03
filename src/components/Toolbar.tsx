import { useSettings } from "../store/settingsStore";

export function Toolbar() {
  const { openSettings } = useSettings();

  return (
    <div className="bg-gray-800 border-b border-gray-700 px-4 py-3 flex items-center justify-between">
      <h1 className="text-lg font-semibold text-white">Session Context Manager</h1>
      <button
        onClick={openSettings}
        className="px-3 py-1.5 text-sm bg-blue-600 hover:bg-blue-700 text-white rounded-md transition-colors"
      >
        Settings
      </button>
    </div>
  );
}
