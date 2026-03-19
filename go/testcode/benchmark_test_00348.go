package testcode

import (
	"log"
	"net/http"
)

func BenchmarkTest00348(w http.ResponseWriter, r *http.Request) {
	log.Printf("Request from: %s", r.RemoteAddr)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
