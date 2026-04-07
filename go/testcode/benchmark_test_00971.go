package testcode

import (
	"fmt"
	"log"
	"net/http"
)

func benchmarkTest00971AuditLog(entry string) {
	log.Println(entry)
}

func BenchmarkTest00971(w http.ResponseWriter, r *http.Request) {
	action := r.FormValue("action")
	username := r.FormValue("username")
	benchmarkTest00971AuditLog(fmt.Sprintf("action=%s user=%s", action, username))
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
