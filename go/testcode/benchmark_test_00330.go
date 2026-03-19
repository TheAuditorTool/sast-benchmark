package testcode

import "net/http"

func BenchmarkTest00330(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	err := BenchSvcQueryUserSafe(id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
