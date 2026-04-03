import { useState, useEffect } from "react";

interface SettingsProps {
  isOpen: boolean;
  onClose: () => void;
  claudePath: string;
  opencodePath: string;
  exportPath: string;
  onRescan: () => void;
  onExportPathChange: (path: string) => void;
}

export default function Settings({
  isOpen,
  onClose,
  claudePath,
  opencodePath,
  exportPath,
  onRescan,
  onExportPathChange,
}: SettingsProps) {
  const [localExportPath, setLocalExportPath] = useState(exportPath);

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
      <div className="bg-white rounded-lg shadow-xl max-w-md w-full mx-4">
        <div className="flex items-center justify-between px-6 py-4 border-b border-gray-200">
          <h2 className="text-lg font-semibold text-gray-900">Settings</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600 transition-colors"
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
            <h3 className="text-sm font-medium text-gray-700">Source Paths</h3>

            <div>
              <label
                htmlFor="claude-path"
                className="block text-xs text-gray-500 mb-1"
              >
                Claude Path
              </label>
              <input
                id="claude-path"
                type="text"
                value={claudePath}
                readOnly
                className="w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50 text-gray-600 text-sm"
              />
            </div>

            <div>
              <label
                htmlFor="opencode-path"
                className="block text-xs text-gray-500 mb-1"
              >
                OpenCode Path
              </label>
              <input
                id="opencode-path"
                type="text"
                value={opencodePath}
                readOnly
                className="w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50 text-gray-600 text-sm"
              />
            </div>
          </div>

          <div className="space-y-3 pt-3 border-t border-gray-200">
            <h3 className="text-sm font-medium text-gray-700">
              Export Path
            </h3>
            <div>
              <label
                htmlFor="export-path"
                className="block text-xs text-gray-500 mb-1"
              >
                Export Directory
              </label>
              <input
                id="export-path"
                type="text"
                value={localExportPath}
                onChange={handleExportPathChange}
                placeholder="/path/to/export"
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm"
              />
            </div>
          </div>
        </div>

        <div className="flex justify-end gap-2 px-6 py-4 border-t border-gray-200 bg-gray-50 rounded-b-lg">
          <button
            onClick={onRescan}
            className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors"
          >
            Rescan
          </button>
          <button
            onClick={onClose}
            className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}
