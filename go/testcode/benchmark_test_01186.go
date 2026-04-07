package testcode

import (
	"net/http"
)

func BenchmarkTest01186(w http.ResponseWriter, r *http.Request) {
	if r.Method == http.MethodGet {
		http.SetCookie(w, &http.Cookie{
			Name:     "session",
			Value:    "sess-abc",
			SameSite: http.SameSiteNoneMode,
			Secure:   true,
		})
		RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
		return
	}
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	userID := r.FormValue("user_id")
	newBio := r.FormValue("bio")
	_, err := DB.Exec("UPDATE profiles SET bio = ? WHERE user_id = ?", newBio, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "profile updated"})
}
