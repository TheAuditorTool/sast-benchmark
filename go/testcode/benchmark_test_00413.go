package testcode

import (
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func BenchmarkTest00413(w http.ResponseWriter, r *http.Request) {
	src := r.FormValue("src")
	dst := r.FormValue("dst")
	absSrc, err := filepath.Abs(src)
	if err != nil || !strings.HasPrefix(absSrc, "/var/data") {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	absDst, err := filepath.Abs(dst)
	if err != nil || !strings.HasPrefix(absDst, "/var/data") {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	err = os.Rename(absSrc, absDst)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "renamed"})
}
