package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00379(w http.ResponseWriter, r *http.Request) {
	badQuery := r.URL.Query().Get("q")
	_, err := DB.Exec(badQuery)
	if err != nil {
		errMsg := err.Error()
		DB.Exec(fmt.Sprintf("INSERT INTO error_log (msg) VALUES ('%s')", errMsg))
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "logged"})
}
