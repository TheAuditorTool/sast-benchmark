package testcode

import "net/http"

func BenchmarkTest01072(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	var id int
	err := DB.QueryRow("SELECT id FROM users WHERE username = ? AND password_hash = ?", username, password).Scan(&id)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	sess, _ := SessionStore.Get(r, "session")
	sess.Values["user_id"] = id
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
