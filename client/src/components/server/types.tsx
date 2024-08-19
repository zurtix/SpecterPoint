export interface Server {
  id: number,
  name: string,
  type: string,
  host: string,
  port: number,
  username: string,
  password: string
}

export interface ServerBase {
  name: string,
  type: string,
  host: string,
  port: number,
  username: string,
  password: string
}
