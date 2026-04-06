package testcode

import (
	"fmt"
	"log"
	"net/http"
)

func BenchmarkTest00602(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	if username == "" {
		http.Error(w, "username required", http.StatusBadRequest)
		return
	}

	fmt.Fprintf(log.Writer(), "Login attempt by %q\n", username)

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
