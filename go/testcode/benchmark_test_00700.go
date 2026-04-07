package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00700(w http.ResponseWriter, r *http.Request) {
	userSrc := r.URL.Query().Get("src")
	linkDst := "/var/app/links/current"

	if err := os.Symlink(userSrc, linkDst); err != nil {
		http.Error(w, "symlink error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "linked"})
}
