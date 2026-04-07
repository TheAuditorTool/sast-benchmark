package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest01330(w http.ResponseWriter, r *http.Request) {
	hostname, _ := os.Hostname()
	w.Header().Set("X-Internal-IP", hostname)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
