package testcode

import (
	"net/http"
)

var benchmarkTest00570AllowedThemes = map[string]bool{
	"light":  true,
	"dark":   true,
	"system": true,
}

func BenchmarkTest00570(w http.ResponseWriter, r *http.Request) {
	theme := r.FormValue("theme")
	if !benchmarkTest00570AllowedThemes[theme] {
		theme = "light"
	}

	session, err := SessionStore.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}

	session.Values["theme"] = theme

	if err := session.Save(r, w); err != nil {
		http.Error(w, "session save error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"theme": theme})
}
