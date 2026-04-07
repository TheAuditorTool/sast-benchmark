package testcode

import "net/http"

func BenchmarkTest01082(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	var userID int
	var tenantID int
	err := DB.QueryRow("SELECT id, tenant_id FROM users WHERE username = ? AND password_hash = ?", username, password).Scan(&userID, &tenantID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	sess, _ := SessionStore.Get(r, "session")
	sess.Values["user_id"] = userID
	sess.Values["tenant_id"] = tenantID
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
