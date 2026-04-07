package testcode

import (
	"log"
	"net/http"
)

func BenchmarkTest00967(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	action := r.FormValue("action")
	log.Printf("user=%s action=%s", username, action)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
