package testcode

import (
	"fmt"
	"net/http"
	"os"
)

func BenchmarkTest00345(w http.ResponseWriter, r *http.Request) {
	action := r.FormValue("action")
	f, err := os.OpenFile("/var/log/app.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer f.Close()
	fmt.Fprintf(f, "Action: %s\n", action)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
