package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00698(w http.ResponseWriter, r *http.Request) {
	var req struct {
		OutputPath string `json:"output_path"`
		Content    string `json:"content"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}
	if err := os.WriteFile(req.OutputPath, []byte(req.Content), 0644); err != nil {
		http.Error(w, "write error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "written"})
}
