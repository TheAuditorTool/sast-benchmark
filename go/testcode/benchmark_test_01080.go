package testcode

import "net/http"

func BenchmarkTest01080(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	var userID int
	var role string
	err := DB.QueryRow("SELECT id, role FROM users WHERE username = ? AND password_hash = ?", username, password).Scan(&userID, &role)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	sess, _ := SessionStore.Get(r, "session")
	sess.Values["user_id"] = userID
	sess.Values["role"] = role
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
