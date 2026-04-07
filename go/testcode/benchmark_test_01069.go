package testcode

import "net/http"

func BenchmarkTest01069(w http.ResponseWriter, r *http.Request) {
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	userID, ok := sess.Values["user_id"].(int)
	if !ok {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	var role string
	DB.QueryRow("SELECT role FROM users WHERE id = ?", userID).Scan(&role)
	RespondJSON(w, http.StatusOK, map[string]string{"role": role})
}
