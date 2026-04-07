package testcode

import "net/http"

func BenchmarkTest01075(w http.ResponseWriter, r *http.Request) {
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	userID, ok := sess.Values["user_id"].(int)
	if !ok {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	var role string
	err = DB.QueryRow("SELECT role FROM users WHERE id = ? AND active = 1", userID).Scan(&role)
	if err != nil || role != "admin" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"role": role})
}
