package testcode

import "net/http"

var benchmarkTest00889Destinations = map[string]string{
	"home":     "/",
	"profile":  "/profile",
	"settings": "/settings",
}

func BenchmarkTest00889(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("to")
	dest, ok := benchmarkTest00889Destinations[key]
	if !ok {
		dest = "/"
	}
	http.Redirect(w, r, dest, http.StatusFound)
}
