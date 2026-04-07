package testcode

import "net/http"

func BenchmarkTest00877(w http.ResponseWriter, r *http.Request) {
	dest := r.FormValue("destination")
	http.Redirect(w, r, dest, http.StatusSeeOther)
}
