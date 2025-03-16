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

export interface WmsSummary {
  id: number,
  name: string,
}

export interface WmsGroup {
  id: number,
  name: string,
  wms: WmsSummary[],
  groups: WmsGroup[],
}
