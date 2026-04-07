package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest01070(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	var userID int
	err := DB.QueryRow("SELECT id FROM users WHERE username = ? AND password_hash = ?", username, password).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	token := make([]byte, 32)
	rand.Read(token)
	tokenStr := hex.EncodeToString(token)
	DB.Exec("INSERT INTO sessions (token, user_id) VALUES (?, ?)", tokenStr, userID)
	sess, _ := SessionStore.Get(r, "session")
	sess.Values["token"] = tokenStr
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
