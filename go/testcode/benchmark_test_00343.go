package testcode

import (
	"log"
	"net/http"
)

func BenchmarkTest00343(w http.ResponseWriter, r *http.Request) {
	user := r.FormValue("username")
	log.Printf("Login attempt: user=%s", user)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
