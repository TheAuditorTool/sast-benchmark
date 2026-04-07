package testcode

import (
	"io"
	"net/http"

	"gopkg.in/yaml.v3"
)

type benchmarkTest00958ServerConfig struct {
	Host    string `yaml:"host"`
	Port    int    `yaml:"port"`
	Timeout int    `yaml:"timeout"`
	Debug   bool   `yaml:"debug"`
}

func BenchmarkTest00958(w http.ResponseWriter, r *http.Request) {
	body, _ := io.ReadAll(io.LimitReader(r.Body, 1<<16))
	var cfg benchmarkTest00958ServerConfig
	if err := yaml.Unmarshal(body, &cfg); err != nil {
		http.Error(w, "yaml error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"host": cfg.Host,
		"port": cfg.Port,
	})
}
