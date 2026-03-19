package testcode

import (
	"net/http"
)

func BenchmarkTest00370(w http.ResponseWriter, r *http.Request) {
	session, _ := SessionStore.Get(r, "app-session")
	key := r.FormValue("key")
	session.Values[key] = "value"
	session.Save(r, w)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
