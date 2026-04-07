package testcode

import "net/http"

func BenchmarkTest01068(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	var userID int
	var storedHash string
	err := DB.QueryRow("SELECT id, password_hash FROM users WHERE username = ?", username).Scan(&userID, &storedHash)
	if err != nil || storedHash != password {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	sess.Values["user_id"] = userID
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "authenticated"})
}
