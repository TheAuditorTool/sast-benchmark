package testcode

import (
	"net/http"
)

func BenchmarkTest00369(w http.ResponseWriter, r *http.Request) {
	session, _ := SessionStore.Get(r, "app-session")
	authToken := r.Header.Get("X-Auth-Token")
	session.Values["auth_token"] = authToken
	session.Save(r, w)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
