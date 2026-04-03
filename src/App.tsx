import { useState, useEffect } from "react";
import { Sidebar } from "./components/Sidebar";
import { Toolbar } from "./components/Toolbar";
import { SessionList } from "./components/SessionList";
import { Preview } from "./components/Preview";
import "./App.css";

function App() {
  const [isScanning, setIsScanning] = useState(true);

  useEffect(() => {
    const timer = setTimeout(() => {
      setIsScanning(false);
    }, 1000);
    return () => clearTimeout(timer);
  }, []);

  if (isScanning) {
    return (
      <div className="flex items-center justify-center h-screen bg-gray-900 text-white">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto mb-4"></div>
          <p className="text-lg">Scanning sessions...</p>
        </div>
      </div>
    );
  }

  return (
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
  );
}

export default App;
