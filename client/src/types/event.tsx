export type Log = {
  level: string,
  message: string,
  timestamp: string
}

export type Agent = {
  id: string,
  type: string
}

export interface Message {
  log: Log,
  agent: Agent
}

export interface Interaction {
  history: string[],
  commands: string[],
  id: string,
  type: string,
}

