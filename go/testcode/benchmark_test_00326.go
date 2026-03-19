package testcode

import "net/http"

func BenchmarkTest00326(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	err := BenchSvcQueryUser(id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
