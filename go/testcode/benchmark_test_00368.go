package testcode

import (
	"net/http"
)

func BenchmarkTest00368(w http.ResponseWriter, r *http.Request) {
	session, _ := SessionStore.Get(r, "app-session")
	prefs := r.URL.Query().Get("prefs")
	session.Values["preferences"] = prefs
	session.Save(r, w)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
