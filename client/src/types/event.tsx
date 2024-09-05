export type Log = {
  level: string,
  message: string,
  timestamp: string
}

export type Agent = {
  id: string,
  last_seen: string
}

export interface Message {
  log?: Log,
  checkin?: Agent
}

export interface Interaction {
  history: string[],
  commands: string[],
  id: string,
  type: string,
}

