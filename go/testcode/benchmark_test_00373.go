package testcode

import (
	"net/http"
)

func BenchmarkTest00373(w http.ResponseWriter, r *http.Request) {
	session, _ := SessionStore.Get(r, "app-session")
	session.Values["theme"] = "dark"
	session.Save(r, w)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
