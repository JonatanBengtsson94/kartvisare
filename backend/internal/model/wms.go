package model

type WmsParams struct {
	Layers  string `json:"LAYERS"`
	Version string `json:"VERSION"`
	Format  string `json:"FORMAT"`
	SRS     string `json:"SRS"`
}

type Wms struct {
	Name   string    `json:"name"`
	Url    string    `json:"url"`
	Params WmsParams `json:"params"`
}
