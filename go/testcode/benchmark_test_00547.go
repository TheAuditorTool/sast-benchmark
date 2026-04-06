package testcode

import (
	"net/http"
	"net/url"
)

var benchmarkTest00547AllowedHosts = map[string]bool{
	"app.example.com":     true,
	"account.example.com": true,
	"api.example.com":     true,
}

func BenchmarkTest00547(w http.ResponseWriter, r *http.Request) {
	redirectTo := r.URL.Query().Get("redirect_to")
	if redirectTo == "" {
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}

	parsed, err := url.Parse(redirectTo)
	if err != nil {
		http.Error(w, "invalid redirect", http.StatusBadRequest)
		return
	}

	if parsed.Host != "" && !benchmarkTest00547AllowedHosts[parsed.Host] {
		http.Error(w, "redirect not allowed", http.StatusBadRequest)
		return
	}

	http.Redirect(w, r, parsed.String(), http.StatusFound)
}
