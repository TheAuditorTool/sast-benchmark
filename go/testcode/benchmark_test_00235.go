package testcode

import (
	"net/http"
)

var redirectMap = map[string]string{
	"dashboard": "/dashboard",
	"profile":   "/user/profile",
	"settings":  "/user/settings",
	"home":      "/",
	"logout":    "/auth/logout",
}

func BenchmarkTest00235(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("dest")
	if key == "" {
		http.Error(w, "missing dest", http.StatusBadRequest)
		return
	}

	path, ok := redirectMap[key]
	if !ok {
		http.Error(w, "unknown destination", http.StatusBadRequest)
		return
	}

	http.Redirect(w, r, path, http.StatusFound)
}
