export interface Listener {
  id: number,
  name: string,
  host: string,
  port: number,
  type: string,
  endpoints: string[]
}

export enum ListenerTypes {
  http = "http",
  https = "http",
  tcp = "tcp",
}

