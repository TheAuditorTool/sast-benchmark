package testcode

import "net/http"

func BenchmarkTest00899(w http.ResponseWriter, r *http.Request) {
	configKey := r.URL.Query().Get("config")
	var redirectURL string
	err := DB.QueryRow("SELECT redirect_url FROM site_config WHERE key = ? AND active = 1", configKey).Scan(&redirectURL)
	if err != nil {
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}
	http.Redirect(w, r, redirectURL, http.StatusFound)
}
