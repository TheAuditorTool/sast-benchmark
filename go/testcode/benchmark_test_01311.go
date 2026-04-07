package testcode

import (
	"math"
	"net/http"
	"strconv"
)

func BenchmarkTest01311(w http.ResponseWriter, r *http.Request) {
	f, err := strconv.ParseFloat(r.FormValue("val"), 64)
	if err != nil || math.IsNaN(f) || math.IsInf(f, 0) {
		http.Error(w, "invalid float", http.StatusBadRequest)
		return
	}
	if f > 0 {
		RespondJSON(w, http.StatusOK, map[string]string{"access": "granted"})
		return
	}
	http.Error(w, "access denied", http.StatusForbidden)
}
