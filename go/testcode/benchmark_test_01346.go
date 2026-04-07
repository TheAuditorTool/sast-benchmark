package testcode

import (
	"net/http"
)

func BenchmarkTest01346(w http.ResponseWriter, r *http.Request) {
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
