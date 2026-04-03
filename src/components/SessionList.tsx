import { useEffect } from 'react';
import { useSessionsStore } from '../hooks/useSessions';

export function SessionList() {
  const {
    filteredSessions,
    selectedSession,
    selectSession,
    isLoading,
    loadSessions
  } = useSessionsStore();

  useEffect(() => {
    loadSessions();
  }, [loadSessions]);

  const formatDate = (isoString: string) => {
    try {
      const date = new Date(isoString);
      return date.toLocaleDateString() + ' ' + date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    } catch {
      return isoString;
    }
  };

  if (isLoading) {
    return (
      <div className="w-80 border-r border-slate-700 bg-slate-900 flex items-center justify-center">
        <div className="text-gray-400">Loading sessions...</div>
      </div>
    );
  }

  return (
    <div className="w-80 border-r border-slate-700 bg-slate-900 flex flex-col h-full">
      <div className="p-3 border-b border-slate-700">
        <h3 className="text-sm font-medium text-gray-300">
          {filteredSessions.length} session{filteredSessions.length !== 1 ? 's' : ''}
        </h3>
      </div>
      <div className="flex-1 overflow-y-auto">
        {filteredSessions.map((session) => (
          <button
            key={session.id}
            onClick={() => selectSession(session)}
            className={`w-full text-left p-3 border-b border-slate-800 transition-colors ${
              selectedSession?.id === session.id
                ? 'bg-blue-900/50 border-l-2 border-l-blue-500'
                : 'hover:bg-slate-800'
            }`}
          >
            <div className="font-medium text-gray-100 truncate text-sm">{session.projectName}</div>
            <div className="text-xs text-gray-400 mt-1">
              {formatDate(session.startedAt)}
            </div>
            <div className="text-xs text-gray-500 mt-1">
              {session.messageCount} messages • {session.source}
            </div>
          </button>
        ))}
        {filteredSessions.length === 0 && (
          <div className="text-gray-500 text-sm p-4 text-center">No sessions found</div>
        )}
      </div>
    </div>
  );
}
