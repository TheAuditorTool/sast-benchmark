package testcode

import (
	"net/http"
	"time"
)

func BenchmarkTest01078(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	var userID int
	err := DB.QueryRow("SELECT id FROM users WHERE username = ? AND password_hash = ?", username, password).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	sess, _ := SessionStore.Get(r, "session")
	sess.Values["user_id"] = userID
	sess.Values["login_time"] = time.Now().Unix()
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
