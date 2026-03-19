package testcode

import (
	"encoding/gob"
	"net/http"
	"os"
)

type cachedSession struct {
	UserID    int    `json:"user_id"`
	Username  string `json:"username"`
	ExpiresAt int64  `json:"expires_at"`
}

func BenchmarkTest00255(w http.ResponseWriter, r *http.Request) {
	file, err := os.Open("/var/cache/sessions/current.gob")
	if err != nil {
		http.Error(w, "session not found", http.StatusNotFound)
		return
	}
	defer file.Close()

	var session cachedSession
	decoder := gob.NewDecoder(file)
	err = decoder.Decode(&session)
	if err != nil {
		http.Error(w, "session decode error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"user_id":  session.UserID,
		"username": session.Username,
	})
}
