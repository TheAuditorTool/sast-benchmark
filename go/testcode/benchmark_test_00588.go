package testcode

import (
	"io"
	"net/http"

	"gopkg.in/yaml.v3"
)

type benchmarkTest00588Config struct {
	Name    string `yaml:"name"`
	Host    string `yaml:"host"`
	Port    int    `yaml:"port"`
	Timeout int    `yaml:"timeout"`
}

func BenchmarkTest00588(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}

	var config benchmarkTest00588Config
	if err := yaml.Unmarshal(body, &config); err != nil {
		http.Error(w, "yaml parse error", http.StatusBadRequest)
		return
	}

	if config.Name == "" {
		http.Error(w, "name is required", http.StatusUnprocessableEntity)
		return
	}
	if config.Port < 1 || config.Port > 65535 {
		http.Error(w, "port out of range", http.StatusUnprocessableEntity)
		return
	}

	RespondJSON(w, http.StatusOK, config)
}
