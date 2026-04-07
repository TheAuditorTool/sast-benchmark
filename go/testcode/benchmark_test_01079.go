package testcode

import "net/http"

func BenchmarkTest01079(w http.ResponseWriter, r *http.Request) {
	email := r.FormValue("email")
	var userID int
	err := DB.QueryRow("SELECT id FROM users WHERE email = ? AND verified = 1", email).Scan(&userID)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	sess, _ := SessionStore.Get(r, "session")
	sess.Values["user_id"] = userID
	sess.Values["email_verified"] = true
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
