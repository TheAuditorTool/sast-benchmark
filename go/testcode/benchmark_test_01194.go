package testcode

import (
	"net/http"
)

func BenchmarkTest01194(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	http.SetCookie(w, &http.Cookie{
		Name:     "session",
		Value:    "sess-xyz",
		SameSite: http.SameSiteStrictMode,
		HttpOnly: true,
		Secure:   true,
	})
	userID := r.URL.Query().Get("user_id")
	var name, email string
	err := DB.QueryRow("SELECT name, email FROM users WHERE id = ?", userID).Scan(&name, &email)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": name, "email": email})
}
