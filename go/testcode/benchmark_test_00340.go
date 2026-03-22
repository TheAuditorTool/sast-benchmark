package testcode

import "net/http"

func BenchmarkTest00340(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("input")
	err := BenchSvcProcessSQL(input)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "processed"})
}
