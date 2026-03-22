package testcode

import (
	"net/http"
)

func BenchmarkTest00481(w http.ResponseWriter, r *http.Request) {
	session, err := SessionStore.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}

	token, ok := session.Values["token"].(string)
	if !ok || token == "" {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var role string
	err = DB.QueryRow("SELECT role FROM users WHERE session_token = ?", token).Scan(&role)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	session.Values["role"] = role
	session.Save(r, w)

	RespondJSON(w, http.StatusOK, map[string]string{"role": role})
}
