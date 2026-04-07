package testcode

import (
	"log"
	"net/http"
	"os"
)

var benchmarkTest00987Logger = log.New(os.Stderr, "[app] ", log.LstdFlags)

func BenchmarkTest00987(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	benchmarkTest00987Logger.Printf("user=%q action=login", username)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
