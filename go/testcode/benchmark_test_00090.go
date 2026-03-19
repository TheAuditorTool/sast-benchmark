package testcode

import (
	"net/http"
	"regexp"
)

func BenchmarkTest00090(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	validName := regexp.MustCompile(`^[a-zA-Z0-9_.-]+$`)
	if !validName.MatchString(filename) {
		http.Error(w, "invalid filename", http.StatusBadRequest)
		return
	}
	http.ServeFile(w, r, "./static/"+filename)
}
