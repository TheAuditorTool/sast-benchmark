package testcode

import (
	"net/http"

	"github.com/google/uuid"
)

func BenchmarkTest00872(w http.ResponseWriter, r *http.Request) {
	sessionID := uuid.New().String()
	_, err := DB.Exec("INSERT INTO sessions (id, user_id) VALUES (?, ?)", sessionID, r.FormValue("user_id"))
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	http.SetCookie(w, &http.Cookie{
		Name:     "session_id",
		Value:    sessionID,
		Secure:   true,
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
		MaxAge:   3600,
		Path:     "/",
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
