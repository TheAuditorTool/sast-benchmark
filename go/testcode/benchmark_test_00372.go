package testcode

import (
	"net/http"
)

func BenchmarkTest00372(w http.ResponseWriter, r *http.Request) {
	session, _ := SessionStore.Get(r, "app-session")
	token := r.Header.Get("Authorization")
	var userID int
	DB.QueryRow("SELECT id FROM users WHERE token = ?", token).Scan(&userID)
	session.Values["user_id"] = userID
	session.Save(r, w)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
