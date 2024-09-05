import { Endpoint } from "./endpoint";

export interface Listener {
  id: number,
  name: string,
  host: string,
  port: number,
  type: string,
  private_key: string,
  public_key: string,
  endpoints: Endpoint[]
}

export enum ListenerTypes {
  http = "http",
  https = "https",
  tcp = "tcp",
}

