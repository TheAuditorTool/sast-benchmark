package testcode

import "net/http"

func BenchmarkTest00873(w http.ResponseWriter, r *http.Request) {
	c, err := r.Cookie("return_url")
	if err != nil {
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}
	http.Redirect(w, r, c.Value, http.StatusFound)
}
