package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00863(w http.ResponseWriter, r *http.Request) {
	userID := r.FormValue("user_id")
	sessionID := make([]byte, 32)
	rand.Read(sessionID)
	sid := hex.EncodeToString(sessionID)
	_, err := DB.Exec("INSERT INTO sessions (id, user_id) VALUES (?, ?)", sid, userID)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	http.SetCookie(w, &http.Cookie{
		Name:     "session_id",
		Value:    sid,
		Secure:   true,
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
		Path:     "/",
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
