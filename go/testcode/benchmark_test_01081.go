package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest01081(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	var userID int
	err := DB.QueryRow("SELECT id FROM users WHERE username = ? AND password_hash = ?", username, password).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	oldSess, _ := SessionStore.Get(r, "session")
	oldSess.Options.MaxAge = -1
	SessionStore.Save(r, w, oldSess)
	b := make([]byte, 32)
	rand.Read(b)
	newToken := hex.EncodeToString(b)
	DB.Exec("INSERT INTO sessions (token, user_id) VALUES (?, ?)", newToken, userID)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
