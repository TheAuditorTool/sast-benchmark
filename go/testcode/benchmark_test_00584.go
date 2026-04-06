package testcode

import (
	"io"
	"net/http"

	"gopkg.in/yaml.v3"
)

func BenchmarkTest00584(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}

	var data map[string]interface{}
	if err := yaml.Unmarshal(body, &data); err != nil {
		http.Error(w, "yaml parse error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, data)
}
