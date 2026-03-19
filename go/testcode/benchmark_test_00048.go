package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00048(w http.ResponseWriter, r *http.Request) {
	xData := r.Header.Get("X-Custom-Data")
	forwardedFor := r.Header.Get("X-Forwarded-For")
	query := fmt.Sprintf(
		"INSERT INTO request_log (custom_data, ip) VALUES ('%s', '%s')",
		xData, forwardedFor,
	)
	DB.Exec(query)
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "logged"})
}
