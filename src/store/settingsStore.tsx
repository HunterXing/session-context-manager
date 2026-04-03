import { createContext, useContext, useState, ReactNode } from "react";

interface SettingsState {
  claudePath: string;
  opencodePath: string;
  exportPath: string;
  isSettingsOpen: boolean;
}

interface SettingsContextType extends SettingsState {
  setClaudePath: (path: string) => void;
  setOpencodePath: (path: string) => void;
  setExportPath: (path: string) => void;
  openSettings: () => void;
  closeSettings: () => void;
  onRescan: () => void;
}

const SettingsContext = createContext<SettingsContextType | null>(null);

export function SettingsProvider({ children }: { children: ReactNode }) {
  const [claudePath, setClaudePath] = useState(
    "/home/user/.claude"
  );
  const [opencodePath, setOpencodePath] = useState(
    "/home/user/.opencode"
  );
  const [exportPath, setExportPath] = useState(
    "/home/user/Documents/sessions"
  );
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);

  const openSettings = () => setIsSettingsOpen(true);
  const closeSettings = () => setIsSettingsOpen(false);

  const onRescan = () => {
    console.log("Rescan triggered - backend handles actual scanning");
  };

  return (
    <SettingsContext.Provider
      value={{
        claudePath,
        opencodePath,
        exportPath,
        isSettingsOpen,
        setClaudePath,
        setOpencodePath,
        setExportPath,
        openSettings,
        closeSettings,
        onRescan,
      }}
    >
      {children}
    </SettingsContext.Provider>
  );
}

export function useSettings() {
  const context = useContext(SettingsContext);
  if (!context) {
    throw new Error("useSettings must be used within SettingsProvider");
  }
  return context;
}
