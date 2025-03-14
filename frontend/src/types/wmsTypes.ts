export interface Wms {
  id: number,
  name: string,
}

export interface WmsGroup {
  id: number,
  name: string,
  wms: Wms[],
  groups: WmsGroup[],
}
