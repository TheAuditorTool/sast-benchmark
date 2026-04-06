package testcode

import (
	"log"
	"net/http"
)

func BenchmarkTest00599(w http.ResponseWriter, r *http.Request) {
	log.Printf("%s %s %s %s", r.RemoteAddr, r.Method, r.URL.Path, r.UserAgent())

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
