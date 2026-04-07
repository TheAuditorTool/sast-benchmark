package testcode

import (
	"net/http"
)

func BenchmarkTest00709(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("username")
	if err != nil {
		http.Error(w, "no cookie", http.StatusBadRequest)
		return
	}
	w.Header().Set("Content-Type", "text/html")
	w.Write([]byte("<html><body><p>Welcome, " + cookie.Value + "</p></body></html>"))
}
