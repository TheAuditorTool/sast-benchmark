package testcode

import (
	"net/http"
)

func BenchmarkTest00568(w http.ResponseWriter, r *http.Request) {
	next := r.FormValue("next")
	if next == "" {
		next = "/dashboard"
	}

	session, err := SessionStore.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}

	session.Values["post_login_redirect"] = next

	if err := session.Save(r, w); err != nil {
		http.Error(w, "session save error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
