package testcode

import (
	"net/http"

	"github.com/gorilla/sessions"
)

var benchmarkTest01199Store = sessions.NewCookieStore([]byte("samesite-lax-session-key-32"))

func BenchmarkTest01199(w http.ResponseWriter, r *http.Request) {
	http.SetCookie(w, &http.Cookie{
		Name:     "session",
		Value:    "sess-lax",
		SameSite: http.SameSiteLaxMode,
		HttpOnly: true,
		Secure:   true,
	})
	if r.Method == http.MethodGet {
		RespondJSON(w, http.StatusOK, map[string]string{"form": "ready"})
		return
	}
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	session, err := benchmarkTest01199Store.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	expected, ok := session.Values["csrf_token"].(string)
	if !ok || expected == "" || r.FormValue("csrf_token") != expected {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	userID := r.FormValue("user_id")
	bio := r.FormValue("bio")
	_, err = DB.Exec("UPDATE profiles SET bio = ? WHERE user_id = ?", bio, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "bio updated"})
}
