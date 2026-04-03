import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { Session, Project } from '../types';

interface SessionsState {
  sessions: Session[];
  filteredSessions: Session[];
  selectedSession: Session | null;
  searchQuery: string;
  isLoading: boolean;
  error: string | null;
  projects: Project[];
  selectedProject: string | null;
  defaultPaths: [string, string][];
}

interface SessionsActions {
  setSearchQuery: (query: string) => void;
  selectSession: (session: Session | null) => void;
  selectProject: (projectName: string | null) => void;
  loadSessions: () => Promise<void>;
  searchSessions: (query: string) => Promise<void>;
  exportSession: (session: Session) => Promise<void>;
  exportProject: (projectName: string) => Promise<void>;
  loadDefaultPaths: () => Promise<void>;
}

type SessionsStore = SessionsState & SessionsActions;

export const useSessionsStore = create<SessionsStore>((set, get) => ({
  sessions: [],
  filteredSessions: [],
  selectedSession: null,
  searchQuery: '',
  isLoading: false,
  error: null,
  projects: [],
  selectedProject: null,
  defaultPaths: [],

  setSearchQuery: (query) => set({ searchQuery: query }),

  selectSession: (session) => set({ selectedSession: session }),

  selectProject: (projectName) => {
    set({ selectedProject: projectName });
    const filtered = projectName
      ? get().sessions.filter(s => s.projectName === projectName)
      : get().sessions;
    set({ filteredSessions: filtered });
  },

  loadSessions: async () => {
    set({ isLoading: true, error: null });
    try {
      const sessions = await invoke<Session[]>('scan');
      const projectsMap = new Map<string, Project>();
      sessions.forEach(s => {
        if (!projectsMap.has(s.projectName)) {
          projectsMap.set(s.projectName, { name: s.projectName, sessionCount: 0, sessions: [] });
        }
        const p = projectsMap.get(s.projectName)!;
        p.sessionCount++;
        p.sessions.push(s);
      });
      const projects = Array.from(projectsMap.values());
      set({ sessions, filteredSessions: sessions, projects, isLoading: false });
    } catch (err) {
      set({ error: String(err), isLoading: false });
    }
  },

  searchSessions: async (query: string) => {
    if (!query.trim()) {
      const { selectedProject, sessions } = get();
      set({ filteredSessions: selectedProject ? sessions.filter(s => s.projectName === selectedProject) : sessions });
      return;
    }
    try {
      const { sessions, selectedProject } = get();
      const searchIn = selectedProject ? sessions.filter(s => s.projectName === selectedProject) : sessions;
      const filtered = await invoke<Session[]>('search', { query, sessions: searchIn });
      set({ filteredSessions: filtered });
    } catch (err) {
      set({ error: String(err) });
    }
  },

  exportSession: async (session: Session) => {
    try {
      await invoke('export_session', { session });
    } catch (err) {
      set({ error: String(err) });
    }
  },

  exportProject: async (projectName: string) => {
    try {
      const { sessions } = get();
      const projectSessions = sessions.filter(s => s.projectName === projectName);
      await invoke('export_project', { project: projectName, sessions: projectSessions });
    } catch (err) {
      set({ error: String(err) });
    }
  },

  loadDefaultPaths: async () => {
    try {
      const paths = await invoke<[string, string][]>('get_default_source_paths');
      set({ defaultPaths: paths });
    } catch (err) {
      console.error('Failed to load default paths:', err);
    }
  },
}));
