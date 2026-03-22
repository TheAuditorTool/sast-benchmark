package testcode

import (
	"encoding/json"
	"io"
	"net/http"
)

type benchmarkTest00490Config struct {
	MaxRetries int    `json:"max_retries"`
	Timeout    int    `json:"timeout"`
	Endpoint   string `json:"endpoint"`
	Enabled    bool   `json:"enabled"`
}

func BenchmarkTest00490(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}

	var config benchmarkTest00490Config
	if err := json.Unmarshal(body, &config); err != nil {
		http.Error(w, "unmarshal error", http.StatusBadRequest)
		return
	}

	if config.MaxRetries < 0 || config.MaxRetries > 10 {
		http.Error(w, "max_retries must be 0-10", http.StatusBadRequest)
		return
	}
	if config.Timeout < 1 || config.Timeout > 300 {
		http.Error(w, "timeout must be 1-300", http.StatusBadRequest)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"status": "configured",
		"config": config,
	})
}
