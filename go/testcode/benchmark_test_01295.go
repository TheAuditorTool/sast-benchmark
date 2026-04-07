package testcode

import (
	"net/http"
	"regexp"
)

func BenchmarkTest01295(w http.ResponseWriter, r *http.Request) {
	userPattern := r.URL.Query().Get("pattern")
	re, err := regexp.Compile(userPattern)
	if err != nil {
		http.Error(w, "invalid pattern", http.StatusBadRequest)
		return
	}
	matched := re.MatchString(r.URL.Query().Get("input"))
	RespondJSON(w, http.StatusOK, map[string]bool{"matched": matched})
}
