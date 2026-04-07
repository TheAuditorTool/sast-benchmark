package testcode

import (
	"net/http"
)

func BenchmarkTest00719(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("msg")
	RespondJSON(w, http.StatusOK, map[string]interface{}{"message": input, "status": "ok"})
}
