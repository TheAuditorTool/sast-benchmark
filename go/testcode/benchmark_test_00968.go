package testcode

import (
	"log"
	"net/http"
)

func benchmarkTest00968LogAction(user, action string) {
	log.Printf("audit: user=" + user + " performed=" + action)
}

func BenchmarkTest00968(w http.ResponseWriter, r *http.Request) {
	benchmarkTest00968LogAction(r.FormValue("username"), r.FormValue("action"))
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
