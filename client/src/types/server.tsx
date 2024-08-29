export interface Server {
  id: number,
  name: string,
  type: string,
  scheme: ServerScheme,
  host: string,
  port: number,
  log_port: number,
  username: string,
  password: string
}

export interface ServerBase {
  name: string,
  type: string,
  scheme: ServerScheme,
  host: string,
  port: number,
  log_port: number,
  username: string,
  password: string
}

export enum ServerScheme {
  http = "http",
  https = "https",
}
