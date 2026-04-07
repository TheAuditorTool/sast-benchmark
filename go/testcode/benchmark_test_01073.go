package testcode

import "net/http"

func BenchmarkTest01073(w http.ResponseWriter, r *http.Request) {
	c, err := r.Cookie("session_token")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	var userID int
	var role string
	err = DB.QueryRow("SELECT user_id, role FROM sessions WHERE token = ?", c.Value).Scan(&userID, &role)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"user_id": userID, "role": role})
}
