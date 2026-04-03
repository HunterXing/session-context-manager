export type SessionSource = 'Claude' | 'OpenCode';

export interface Message {
  role: 'user' | 'assistant';
  content: string;
  timestamp: string; // ISO8601
}

export interface Session {
  id: string;
  source: SessionSource;
  projectPath: string;
  projectName: string;
  startedAt: string; // ISO8601
  updatedAt: string; // ISO8601
  messages: Message[];
  messageCount: number;
}

export interface Project {
  name: string;
  sessionCount: number;
  sessions: Session[];
}
