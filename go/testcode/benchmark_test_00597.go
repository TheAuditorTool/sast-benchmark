package testcode

import (
	"fmt"
	"log"
	"net/http"
	"time"
)

func BenchmarkTest00597(w http.ResponseWriter, r *http.Request) {
	action := r.FormValue("action")
	userID := r.FormValue("user_id")

	if action == "" || userID == "" {
		http.Error(w, "action and user_id required", http.StatusBadRequest)
		return
	}

	fmt.Fprintf(log.Writer(), "[%s] user=%s performed: %s\n", time.Now().Format(time.RFC3339), userID, action)

	RespondJSON(w, http.StatusOK, map[string]string{"status": "logged"})
}
