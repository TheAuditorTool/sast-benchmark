package testcode

import (
	"net/http"
	"time"
)

var benchmarkTest00623AllowedOrigins = map[string]bool{
	"https://example.com":     true,
	"https://app.example.com": true,
}

func BenchmarkTest00623(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	origin := r.Header.Get("Origin")
	if !benchmarkTest00623AllowedOrigins[origin] {
		http.Error(w, "forbidden origin", http.StatusForbidden)
		return
	}

	cookie, err := r.Cookie("session")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID int
	err = DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", cookie.Value).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var req struct {
		DisplayName string `json:"display_name"`
		Bio         string `json:"bio"`
		Timezone    string `json:"timezone"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	_, err = DB.Exec(
		"UPDATE users SET display_name = ?, bio = ?, timezone = ? WHERE id = ?",
		req.DisplayName, req.Bio, req.Timezone, userID,
	)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}

	http.SetCookie(w, &http.Cookie{
		Name:     "session",
		Value:    cookie.Value,
		Path:     "/",
		HttpOnly: true,
		Secure:   true,
		SameSite: http.SameSiteStrictMode,
		Expires:  time.Now().Add(24 * time.Hour),
	})

	RespondJSON(w, http.StatusOK, map[string]string{"status": "settings updated"})
}
