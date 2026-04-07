package testcode

import "net/http"

var benchmarkTest00902Allowed = map[string]bool{
	"https://partner1.example.com/callback": true,
	"https://partner2.example.com/oauth":    true,
	"https://internal.example.com/return":   true,
}

func BenchmarkTest00902(w http.ResponseWriter, r *http.Request) {
	dest := r.URL.Query().Get("dest")
	if !benchmarkTest00902Allowed[dest] {
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}
	http.Redirect(w, r, dest, http.StatusFound)
}
