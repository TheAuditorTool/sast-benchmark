package testcode

import (
	"net/http"
	"os"

	"gopkg.in/yaml.v3"
)

type benchmarkTest01107AppConfig struct {
	Database struct {
		Password string `yaml:"password"`
	} `yaml:"database"`
}

func BenchmarkTest01107(w http.ResponseWriter, r *http.Request) {
	cfgPath := os.Getenv("CONFIG_PATH")
	data, err := os.ReadFile(cfgPath)
	if err != nil {
		http.Error(w, "config not found", http.StatusInternalServerError)
		return
	}
	var cfg benchmarkTest01107AppConfig
	yaml.Unmarshal(data, &cfg)
	RespondJSON(w, http.StatusOK, map[string]bool{"db_configured": cfg.Database.Password != ""})
}
