import { useState, useEffect } from "react";
import { useSessionsStore } from "../hooks/useSessions";

interface SettingsProps {
  isOpen: boolean;
  onClose: () => void;
  exportPath: string;
  onRescan: () => void;
  onExportPathChange: (path: string) => void;
}

export default function Settings({
  isOpen,
  onClose,
  exportPath,
  onRescan,
  onExportPathChange,
}: SettingsProps) {
  const [localExportPath, setLocalExportPath] = useState(exportPath);
  const {
    defaultPaths,
    customPaths,
    loadDefaultPaths,
    loadConfig,
    addCustomPath,
    removeCustomPath,
    updateCustomPath,
  } = useSessionsStore();

  const [newPath, setNewPath] = useState("");
  const [newSourceType, setNewSourceType] = useState<"Claude" | "OpenCode">("Claude");
  const [newName, setNewName] = useState("");

  useEffect(() => {
    if (isOpen) {
      loadDefaultPaths();
      loadConfig();
    }
  }, [isOpen, loadDefaultPaths, loadConfig]);

  useEffect(() => {
    setLocalExportPath(exportPath);
  }, [exportPath]);

  if (!isOpen) return null;

  const handleExportPathChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setLocalExportPath(e.target.value);
    onExportPathChange(e.target.value);
  };

  const handleAddPath = () => {
    if (!newPath.trim()) return;
    addCustomPath({
      name: newName.trim() || newSourceType,
      path: newPath.trim(),
      source_type: newSourceType,
      enabled: true,
    });
    setNewPath("");
    setNewName("");
  };

  const handleToggleEnabled = (index: number) => {
    const path = customPaths[index];
    updateCustomPath(index, { ...path, enabled: !path.enabled });
  };

  return (
    <div
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div className="bg-slate-800 rounded-lg shadow-xl max-w-2xl w-full mx-4 border border-slate-700 max-h-[80vh] flex flex-col">
        <div className="flex items-center justify-between px-6 py-4 border-b border-slate-700">
          <h2 className="text-lg font-semibold text-gray-100">Settings</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-200 transition-colors"
            aria-label="Close settings"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="h-5 w-5"
              viewBox="0 0 20 20"
              fill="currentColor"
            >
              <path
                fillRule="evenodd"
                d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                clipRule="evenodd"
              />
            </svg>
          </button>
        </div>

        <div className="flex-1 overflow-y-auto px-6 py-4 space-y-6">
          <div className="space-y-3">
            <h3 className="text-sm font-medium text-gray-300">Auto-detected Paths</h3>
            <p className="text-xs text-gray-500">
              Automatically detected session source locations
            </p>

            {defaultPaths.length > 0 ? (
              <div className="space-y-2">
                {defaultPaths.map(([name, path]) => (
                  <div key={name} className="flex items-center gap-2 p-2 bg-slate-900 rounded-md">
                    <span className="text-xs px-2 py-0.5 bg-blue-600 rounded text-white">{name}</span>
                    <code className="flex-1 text-xs text-gray-300 font-mono truncate">{path}</code>
                  </div>
                ))}
              </div>
            ) : (
              <div className="text-sm text-gray-500 italic">
                No paths detected. Add custom paths below.
              </div>
            )}
          </div>

          <div className="space-y-3 pt-3 border-t border-slate-700">
            <h3 className="text-sm font-medium text-gray-300">Custom Paths</h3>
            <p className="text-xs text-gray-500">
              Manually add session source paths (e.g., WSL2 paths, custom locations)
            </p>

            <div className="space-y-2">
              {customPaths.map((source, index) => (
                <div key={index} className="flex items-center gap-2 p-2 bg-slate-900 rounded-md">
                  <input
                    type="checkbox"
                    checked={source.enabled}
                    onChange={() => handleToggleEnabled(index)}
                    className="w-4 h-4 rounded border-slate-600 bg-slate-700 text-blue-600"
                  />
                  <span className="text-xs px-2 py-0.5 bg-slate-600 rounded text-gray-300">{source.source_type}</span>
                  <input
                    type="text"
                    value={source.name}
                    onChange={(e) => updateCustomPath(index, { ...source, name: e.target.value })}
                    placeholder="Name"
                    className="w-24 px-2 py-1 text-xs border border-slate-600 rounded bg-slate-800 text-gray-200"
                  />
                  <input
                    type="text"
                    value={source.path}
                    onChange={(e) => updateCustomPath(index, { ...source, path: e.target.value })}
                    placeholder="/path/to/sessions"
                    className="flex-1 px-2 py-1 text-xs border border-slate-600 rounded bg-slate-800 text-gray-200 font-mono"
                  />
                  <button
                    onClick={() => removeCustomPath(index)}
                    className="text-gray-400 hover:text-red-400 transition-colors p-1"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
              ))}
            </div>

            <div className="flex items-center gap-2 pt-2">
              <select
                value={newSourceType}
                onChange={(e) => setNewSourceType(e.target.value as "Claude" | "OpenCode")}
                className="px-2 py-1.5 text-sm border border-slate-600 rounded bg-slate-900 text-gray-200"
              >
                <option value="Claude">Claude</option>
                <option value="OpenCode">OpenCode</option>
              </select>
              <input
                type="text"
                value={newName}
                onChange={(e) => setNewName(e.target.value)}
                placeholder="Name (optional)"
                className="w-32 px-2 py-1.5 text-sm border border-slate-600 rounded bg-slate-900 text-gray-200"
              />
              <input
                type="text"
                value={newPath}
                onChange={(e) => setNewPath(e.target.value)}
                placeholder="/path/to/sessions"
                className="flex-1 px-2 py-1.5 text-sm border border-slate-600 rounded bg-slate-900 text-gray-200 font-mono"
              />
              <button
                onClick={handleAddPath}
                className="px-3 py-1.5 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
              >
                Add
              </button>
            </div>
          </div>

          <div className="space-y-3 pt-3 border-t border-slate-700">
            <h3 className="text-sm font-medium text-gray-300">Export Path</h3>
            <div>
              <label htmlFor="export-path" className="block text-xs text-gray-400 mb-1">
                Export Directory
              </label>
              <input
                id="export-path"
                type="text"
                value={localExportPath}
                onChange={handleExportPathChange}
                placeholder="~/Documents/SessionManager"
                className="w-full px-3 py-2 border border-slate-600 rounded-md bg-slate-900 text-gray-200 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              />
            </div>
          </div>
        </div>

        <div className="flex justify-end gap-2 px-6 py-4 border-t border-slate-700 bg-slate-900 rounded-b-lg">
          <button
            onClick={onRescan}
            className="px-4 py-2 text-sm font-medium text-gray-200 bg-slate-700 border border-slate-600 rounded-md hover:bg-slate-600 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors"
          >
            Rescan
          </button>
          <button
            onClick={onClose}
            className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}
