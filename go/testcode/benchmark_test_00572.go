package testcode

import (
	"net/http"
	"net/url"
)

func BenchmarkTest00572(w http.ResponseWriter, r *http.Request) {
	next := r.FormValue("next")
	if next == "" {
		next = "/dashboard"
	}

	parsed, err := url.Parse(next)
	if err != nil || parsed.Host != "" {
		http.Error(w, "invalid redirect target", http.StatusBadRequest)
		return
	}

	session, err := SessionStore.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}

	session.Values["post_login_redirect"] = parsed.RequestURI()

	if err := session.Save(r, w); err != nil {
		http.Error(w, "session save error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
