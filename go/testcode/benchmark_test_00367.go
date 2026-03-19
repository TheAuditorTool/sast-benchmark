package testcode

import (
	"net/http"

	"github.com/gorilla/sessions"
)

var SessionStore = sessions.NewCookieStore([]byte("benchmark-secret-key"))

func BenchmarkTest00367(w http.ResponseWriter, r *http.Request) {
	session, _ := SessionStore.Get(r, "app-session")
	role := r.FormValue("role")
	session.Values["role"] = role
	session.Save(r, w)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
