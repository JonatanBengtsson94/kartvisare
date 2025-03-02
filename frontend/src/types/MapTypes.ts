export interface WmsParams {
  LAYERS: string;
  VERSION: string;
  FORMAT: string;
  SRS: string;
}

export interface WmsLayer {
  name: string;
  url: string;
  params: WmsParams;
}
