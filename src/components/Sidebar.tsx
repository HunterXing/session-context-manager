import { useSessionsStore } from '../hooks/useSessions';

export function Sidebar() {
  const { projects, selectedProject, selectProject } = useSessionsStore();

  return (
    <div className="w-56 bg-slate-800 border-r border-slate-700 flex flex-col h-full">
      <div className="p-4 border-b border-slate-700">
        <h2 className="text-lg font-semibold text-gray-100">Projects</h2>
      </div>
      <div className="flex-1 overflow-y-auto p-2">
        <button
          onClick={() => selectProject(null)}
          className={`w-full text-left px-3 py-2 rounded-lg mb-1 transition-colors ${
            selectedProject === null
              ? 'bg-blue-600 text-white'
              : 'text-gray-300 hover:bg-slate-700'
          }`}
        >
          All Projects
        </button>
        {projects.map((project) => (
          <button
            key={project.name}
            onClick={() => selectProject(project.name)}
            className={`w-full text-left px-3 py-2 rounded-lg mb-1 transition-colors ${
              selectedProject === project.name
                ? 'bg-blue-600 text-white'
                : 'text-gray-300 hover:bg-slate-700'
            }`}
          >
            <div className="font-medium truncate">{project.name}</div>
            <div className="text-xs opacity-70">{project.sessionCount} sessions</div>
          </button>
        ))}
        {projects.length === 0 && (
          <div className="text-gray-500 text-sm p-3">No sessions found</div>
        )}
      </div>
    </div>
  );
}
