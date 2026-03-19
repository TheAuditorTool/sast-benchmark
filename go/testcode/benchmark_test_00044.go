package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00044(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("input")
	go func(data string) {
		query := fmt.Sprintf("INSERT INTO async_logs (data) VALUES ('%s')", data)
		DB.Exec(query)
	}(input)
	RespondJSON(w, http.StatusAccepted, map[string]string{"status": "queued"})
}
