import ReactMarkdown from 'react-markdown';
import { useSessionsStore } from '../hooks/useSessions';

export function Preview() {
  const { selectedSession, exportSession } = useSessionsStore();

  if (!selectedSession) {
    return (
      <div className="flex-1 bg-slate-900 flex items-center justify-center">
        <div className="text-gray-500">Select a session to preview</div>
      </div>
    );
  }

  const formatDate = (isoString: string) => {
    try {
      return new Date(isoString).toLocaleString();
    } catch {
      return isoString;
    }
  };

  return (
    <div className="flex-1 bg-slate-900 flex flex-col h-full">
      <div className="p-4 border-b border-slate-700 flex items-center justify-between">
        <div>
          <h3 className="font-medium text-gray-100">{selectedSession.projectName}</h3>
          <div className="text-xs text-gray-400 mt-1">
            {formatDate(selectedSession.startedAt)} • {selectedSession.messageCount} messages
          </div>
        </div>
        <button
          onClick={() => exportSession(selectedSession)}
          className="px-3 py-1.5 text-sm bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
        >
          Export
        </button>
      </div>
      <div className="flex-1 overflow-y-auto p-4">
        {selectedSession.messages.map((msg, idx) => (
          <div
            key={idx}
            className={`mb-4 p-3 rounded-lg ${
              msg.role === 'user'
                ? 'bg-blue-900/30 ml-8'
                : 'bg-slate-800 mr-8'
            }`}
          >
            <div className="text-xs font-medium text-gray-400 mb-1 uppercase">
              {msg.role}
            </div>
            <div className="text-gray-200 text-sm prose prose-invert prose-sm max-w-none">
              <ReactMarkdown>{msg.content}</ReactMarkdown>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
