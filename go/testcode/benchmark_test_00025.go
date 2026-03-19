package testcode

import (
	"fmt"
	"io"
	"net/http"
)

func BenchmarkTest00025(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	query := fmt.Sprintf("INSERT INTO raw_logs (data) VALUES ('%s')", string(body))
	_, err = DB.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "logged"})
}
