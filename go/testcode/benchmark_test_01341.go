package testcode

import (
	"net/http"
)

type benchmarkTest01341APIError struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
	detail  string
}

func BenchmarkTest01341(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var name string
	err := DB.QueryRow("SELECT name FROM users WHERE id = ?", id).Scan(&name)
	if err != nil {
		apiErr := benchmarkTest01341APIError{
			Code:    500,
			Message: "request could not be completed",
			detail:  err.Error(),
		}
		RespondJSON(w, http.StatusInternalServerError, apiErr)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
