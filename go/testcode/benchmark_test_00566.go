package testcode

import (
	"net/http"
)

func BenchmarkTest00566(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("theme")
	if err != nil {
		http.Error(w, "missing theme cookie", http.StatusBadRequest)
		return
	}

	session, err := SessionStore.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}

	session.Values["theme"] = cookie.Value

	if err := session.Save(r, w); err != nil {
		http.Error(w, "session save error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
