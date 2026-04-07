package testcode

import "net/http"

func BenchmarkTest00887(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	returnTo := r.FormValue("return_to")
	var storedPw string
	err := DB.QueryRow("SELECT password FROM users WHERE username = ?", username).Scan(&storedPw)
	if err != nil || storedPw != password {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	http.Redirect(w, r, returnTo, http.StatusFound)
}
