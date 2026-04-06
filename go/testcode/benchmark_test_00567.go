package testcode

import (
	"net/http"
)

func BenchmarkTest00567(w http.ResponseWriter, r *http.Request) {
	scope := r.Header.Get("X-API-Scope")
	if scope == "" {
		http.Error(w, "missing scope header", http.StatusBadRequest)
		return
	}

	session, err := SessionStore.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}

	session.Values["api_scope"] = scope

	if err := session.Save(r, w); err != nil {
		http.Error(w, "session save error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
