package testcode

import (
	"flag"
	"net/http"
)

var benchmarkTest01102DBPassword = flag.String("db-password", "", "database password")

func BenchmarkTest01102(w http.ResponseWriter, r *http.Request) {
	if *benchmarkTest01102DBPassword == "" {
		http.Error(w, "db-password flag not set", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"configured": "true"})
}
