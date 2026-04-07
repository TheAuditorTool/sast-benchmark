package testcode

import (
	"net/http"
	"regexp"
)

var benchmarkTest01314Pattern = regexp.MustCompile(`^[a-zA-Z0-9._+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$`)

func BenchmarkTest01314(w http.ResponseWriter, r *http.Request) {
	email := r.FormValue("email")
	if !benchmarkTest01314Pattern.MatchString(email) {
		http.Error(w, "invalid email format", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"email": email})
}
