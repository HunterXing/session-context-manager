import { useSessionsStore } from '../hooks/useSessions';

export function SearchBar() {
  const { searchQuery, setSearchQuery, searchSessions } = useSessionsStore();

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const query = e.target.value;
    setSearchQuery(query);
    searchSessions(query);
  };

  return (
    <div className="relative flex-1 max-w-md">
      <input
        type="text"
        value={searchQuery}
        onChange={handleChange}
        placeholder="Search sessions..."
        className="w-full pl-10 pr-4 py-2 border border-gray-600 rounded-lg bg-slate-800 text-gray-100 placeholder-gray-400 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none"
      />
      <svg
        className="absolute left-3 top-2.5 w-5 h-5 text-gray-400"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
      </svg>
    </div>
  );
}
