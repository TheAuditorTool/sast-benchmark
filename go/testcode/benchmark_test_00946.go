package testcode

import (
	"net/http"

	"gopkg.in/yaml.v3"
)

func BenchmarkTest00946(w http.ResponseWriter, r *http.Request) {
	var data interface{}
	if err := yaml.NewDecoder(r.Body).Decode(&data); err != nil {
		http.Error(w, "yaml error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"data": data})
}
