package testcode

import "net/http"

const benchmarkTest01104Placeholder = "REPLACE_WITH_ACTUAL_SECRET"

func BenchmarkTest01104(w http.ResponseWriter, r *http.Request) {
	if benchmarkTest01104Placeholder == "REPLACE_WITH_ACTUAL_SECRET" {
		http.Error(w, "credential not configured", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
