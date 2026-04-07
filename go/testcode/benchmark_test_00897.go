package testcode

import "net/http"

func BenchmarkTest00897(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	var stored string
	err := DB.QueryRow("SELECT password_hash FROM users WHERE username = ?", username).Scan(&stored)
	if err != nil || stored != password {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	http.Redirect(w, r, "/dashboard", http.StatusFound)
}
