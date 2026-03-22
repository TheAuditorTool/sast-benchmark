package testcode

import (
	"crypto/subtle"
	"net/http"
)

func BenchmarkTest00503(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	cookie, err := r.Cookie("session_id")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID, storedCSRF string
	err = DB.QueryRow("SELECT user_id, csrf_token FROM sessions WHERE token = ?", cookie.Value).Scan(&userID, &storedCSRF)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	submittedCSRF := r.FormValue("csrf_token")
	if subtle.ConstantTimeCompare([]byte(submittedCSRF), []byte(storedCSRF)) != 1 {
		http.Error(w, "csrf validation failed", http.StatusForbidden)
		return
	}

	newEmail := r.FormValue("email")
	_, err = DB.Exec("UPDATE users SET email = ? WHERE id = ?", newEmail, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}

	http.SetCookie(w, &http.Cookie{
		Name:     "session_id",
		Value:    cookie.Value,
		SameSite: http.SameSiteStrictMode,
		HttpOnly: true,
		Secure:   true,
		Path:     "/",
	})

	RespondJSON(w, http.StatusOK, map[string]string{"status": "email_updated"})
}
