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
    claudePath,
    opencodePath,
    exportPath,
    onRescan,
    setExportPath,
  } = useSettings();

  return (
    <>
      <div className="flex h-screen bg-gray-900 text-white">
        <Sidebar />
        <div className="flex flex-col flex-1 overflow-hidden">
          <Toolbar />
          <div className="flex flex-1 overflow-hidden">
            <SessionList />
            <Preview />
          </div>
        </div>
      </div>
      <Settings
        isOpen={isSettingsOpen}
        onClose={closeSettings}
        claudePath={claudePath}
        opencodePath={opencodePath}
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
