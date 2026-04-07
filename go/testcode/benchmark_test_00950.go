package testcode

import (
	"bytes"
	"encoding/gob"
	"net/http"
)

type benchmarkTest00950SessionData struct {
	UserID int
	Role   string
}

func BenchmarkTest00950(w http.ResponseWriter, r *http.Request) {
	sessID := r.Header.Get("X-Session-ID")
	var blob []byte
	err := DB.QueryRow("SELECT data FROM server_sessions WHERE id = ?", sessID).Scan(&blob)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	var sd benchmarkTest00950SessionData
	if err := gob.NewDecoder(bytes.NewReader(blob)).Decode(&sd); err != nil {
		http.Error(w, "decode error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"user_id": sd.UserID, "role": sd.Role})
}
