package testcode

import (
	"encoding/json"
	"net/http"
	"os"
	"path/filepath"
)

func BenchmarkTest00107(w http.ResponseWriter, r *http.Request) {
	var input struct {
		OutputDir  string `json:"output_dir"`
		ReportName string `json:"report_name"`
	}
	json.NewDecoder(r.Body).Decode(&input)
	outputPath := filepath.Join(input.OutputDir, input.ReportName+".json")
	os.WriteFile(outputPath, []byte(`{"status":"complete"}`), 0644)
	RespondJSON(w, http.StatusCreated, map[string]string{"path": outputPath})
}
