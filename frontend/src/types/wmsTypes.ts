export interface Wms {
  id: number,
  name: string,
  description: string,
  layers: string[],
  url: string,
  version: string,
  isActive: boolean,
  authType: string,
  authPassword: string,
  authUsername: string,
}

export interface WmsGroup {
  id: number,
  name: string,
  wms: Wms[],
  groups: WmsGroup[],
}
