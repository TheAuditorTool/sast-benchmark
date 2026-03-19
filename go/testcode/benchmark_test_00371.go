package testcode

import (
	"net/http"
)

func BenchmarkTest00371(w http.ResponseWriter, r *http.Request) {
	session, _ := SessionStore.Get(r, "app-session")
	session.Values["role"] = "user"
	session.Save(r, w)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
