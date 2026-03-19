package testcode

import (
	"net/http"
)

func BenchmarkTest00374(w http.ResponseWriter, r *http.Request) {
	session, _ := SessionStore.Get(r, "app-session")
	val := r.FormValue("role")
	val = "default"
	session.Values["role"] = val
	session.Save(r, w)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
