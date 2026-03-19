package testcode

import (
	"fmt"
	"net/http"
)

func benchmarkTest00009Helper(input string) string {
	return fmt.Sprintf("SELECT * FROM logs WHERE message = '%s'", input)
}

func BenchmarkTest00009(w http.ResponseWriter, r *http.Request) {
	data := r.URL.Query().Get("data")
	query := benchmarkTest00009Helper(data)
	_, err := DB.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "logged"})
}
