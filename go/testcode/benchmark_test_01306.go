package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01306(w http.ResponseWriter, r *http.Request) {
	f, _ := strconv.ParseFloat(r.FormValue("val"), 64)
	if f > 0 {
		RespondJSON(w, http.StatusOK, map[string]string{"access": "granted"})
		return
	}
	http.Error(w, "access denied", http.StatusForbidden)
}
