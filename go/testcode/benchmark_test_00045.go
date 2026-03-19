package testcode

import (
	"net/http"
)

func BenchmarkTest00045(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("input")
	go func(data string) {
		DB.Exec("INSERT INTO async_logs (data) VALUES (?)", data)
	}(input)
	RespondJSON(w, http.StatusAccepted, map[string]string{"status": "queued"})
}
