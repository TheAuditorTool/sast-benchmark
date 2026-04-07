package testcode

import (
	"log"
	"net/http"
	"strconv"
)

func BenchmarkTest00983(w http.ResponseWriter, r *http.Request) {
	userIDStr := r.URL.Query().Get("user_id")
	userID, err := strconv.Atoi(userIDStr)
	if err != nil {
		http.Error(w, "invalid user id", http.StatusBadRequest)
		return
	}
	log.Printf("action by user_id=%d", userID)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
