package testcode

import (
	"net/http"
)

func BenchmarkTest00479(w http.ResponseWriter, r *http.Request) {
	session, err := SessionStore.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}

	xRole := r.Header.Get("X-Role")
	session.Values["role"] = xRole
	session.Save(r, w)

	RespondJSON(w, http.StatusOK, map[string]string{"role": xRole})
}
