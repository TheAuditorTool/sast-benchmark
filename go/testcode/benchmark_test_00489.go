package testcode

import (
	"encoding/gob"
	"net/http"
	"os"
)

type benchmarkTest00489Session struct {
	UserID    string
	Username  string
	ExpiresAt int64
}

func BenchmarkTest00489(w http.ResponseWriter, r *http.Request) {
	sessionFile, err := os.Open("/var/sessions/current.gob")
	if err != nil {
		http.Error(w, "session file not found", http.StatusInternalServerError)
		return
	}
	defer sessionFile.Close()

	var session benchmarkTest00489Session
	decoder := gob.NewDecoder(sessionFile)
	if err := decoder.Decode(&session); err != nil {
		http.Error(w, "decode error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"user_id":  session.UserID,
		"username": session.Username,
	})
}
