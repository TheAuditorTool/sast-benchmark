package testcode

import (
	"io"
	"net/http"

	"gopkg.in/yaml.v3"
)

type benchmarkTest00949Config struct {
	Host string `yaml:"host"`
	Port int    `yaml:"port"`
}

func BenchmarkTest00949(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}
	var cfg benchmarkTest00949Config
	if err := yaml.Unmarshal(body, &cfg); err != nil {
		http.Error(w, "yaml error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"host": cfg.Host, "port": cfg.Port})
}
