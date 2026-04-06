package testcode

import (
	"net/http"
	"net/url"
)

func BenchmarkTest00545(w http.ResponseWriter, r *http.Request) {
	returnURL := r.URL.Query().Get("returnURL")
	if returnURL == "" {
		http.Redirect(w, r, "/dashboard", http.StatusFound)
		return
	}

	cookie, err := r.Cookie("session")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID string
	err = DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", cookie.Value).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	parsedURL, err := url.Parse(returnURL)
	if err != nil {
		http.Redirect(w, r, "/dashboard", http.StatusFound)
		return
	}

	http.Redirect(w, r, parsedURL.String(), http.StatusFound)
}
