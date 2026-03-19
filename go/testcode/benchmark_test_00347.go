package testcode

import (
	"log"
	"net/http"
	"strings"
)

func BenchmarkTest00347(w http.ResponseWriter, r *http.Request) {
	user := r.FormValue("user")
	clean := strings.ReplaceAll(strings.ReplaceAll(user, "\n", ""), "\r", "")
	log.Printf("Login: %s", clean)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
