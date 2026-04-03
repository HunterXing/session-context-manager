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
  const { defaultPaths, loadDefaultPaths } = useSessionsStore();

  useEffect(() => {
    if (isOpen) {
      loadDefaultPaths();
    }
  }, [isOpen, loadDefaultPaths]);

  useEffect(() => {
    setLocalExportPath(exportPath);
  }, [exportPath]);

  if (!isOpen) return null;

  const handleExportPathChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setLocalExportPath(e.target.value);
    onExportPathChange(e.target.value);
  };

  return (
    <div
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div className="bg-slate-800 rounded-lg shadow-xl max-w-md w-full mx-4 border border-slate-700">
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

        <div className="px-6 py-4 space-y-4">
          <div className="space-y-3">
            <h3 className="text-sm font-medium text-gray-300">Source Paths (Auto-detected)</h3>
            <p className="text-xs text-gray-500">
              Sessions are scanned from these paths. On Windows with WSL2, these point to Ubuntu.
            </p>

            {defaultPaths.length > 0 ? (
              defaultPaths.map(([name, path]) => (
                <div key={name}>
                  <label
                    htmlFor={`${name.toLowerCase()}-path`}
                    className="block text-xs text-gray-400 mb-1"
                  >
                    {name}
                  </label>
                  <input
                    id={`${name.toLowerCase()}-path`}
                    type="text"
                    value={path}
                    readOnly
                    className="w-full px-3 py-2 border border-slate-600 rounded-md bg-slate-900 text-gray-300 text-sm font-mono"
                  />
                </div>
              ))
            ) : (
              <div className="text-sm text-gray-500 italic">No paths detected. Make sure Claude Code or OpenCode sessions exist.</div>
            )}
          </div>

          <div className="space-y-3 pt-3 border-t border-slate-700">
            <h3 className="text-sm font-medium text-gray-300">Export Path</h3>
            <div>
              <label
                htmlFor="export-path"
                className="block text-xs text-gray-400 mb-1"
              >
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
