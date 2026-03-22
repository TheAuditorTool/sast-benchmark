package testcode

import (
	"fmt"
	"net/http"
	"os"
)

func BenchmarkTest00496(w http.ResponseWriter, r *http.Request) {
	action := r.FormValue("action")
	userID := r.FormValue("user_id")

	logFile, err := os.OpenFile("/var/log/app/audit.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		http.Error(w, "log error", http.StatusInternalServerError)
		return
	}
	defer logFile.Close()

	fmt.Fprintf(logFile, "action: "+action+" user: "+userID+"\n")

	RespondJSON(w, http.StatusOK, map[string]string{"status": "logged"})
}
