package model

type Wms struct {
	ID          int      `json:"id"`
	Name        string   `json:"name"`
	Description string   `json:"description"`
	Layers      []string `json:"layers"`
	Url         string   `json:"url"`
	Version     string   `json:"version"`
	IsActive    bool     `json:"is_active"`
	AuthType    string   `json:"auth_type"`
	Username    string   `json:"username"`
	Password    string   `json:"password"`
}
