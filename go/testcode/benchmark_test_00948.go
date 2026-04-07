package testcode

import (
	"encoding/json"
	"io"
	"net/http"
)

type benchmarkTest00948Request struct {
	Name  string `json:"name"`
	Email string `json:"email"`
}

func BenchmarkTest00948(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}
	var req benchmarkTest00948Request
	if err := json.Unmarshal(body, &req); err != nil {
		http.Error(w, "json error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": req.Name})
}
