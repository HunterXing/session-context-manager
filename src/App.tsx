import { Sidebar } from "./components/Sidebar";
import { Toolbar } from "./components/Toolbar";
import { SessionList } from "./components/SessionList";
import { Preview } from "./components/Preview";
import Settings from "./components/Settings";
import { SettingsProvider, useSettings } from "./store/settingsStore";
import "./App.css";

function AppContent() {
  const {
    isSettingsOpen,
    closeSettings,
    exportPath,
    onRescan,
    setExportPath,
  } = useSettings();

  return (
    <>
      <div className="flex h-screen bg-slate-900 text-gray-100">
        <Sidebar />
        <div className="flex-1 flex flex-col">
          <Toolbar />
          <div className="flex-1 flex">
            <SessionList />
            <Preview />
          </div>
        </div>
      </div>
      <Settings
        isOpen={isSettingsOpen}
        onClose={closeSettings}
        exportPath={exportPath}
        onRescan={onRescan}
        onExportPathChange={setExportPath}
      />
    </>
  );
}

function App() {
  return (
    <SettingsProvider>
      <AppContent />
    </SettingsProvider>
  );
}

export default App;
