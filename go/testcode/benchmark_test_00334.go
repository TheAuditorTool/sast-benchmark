package testcode

import "net/http"

func BenchmarkTest00334(w http.ResponseWriter, r *http.Request) {
	filter := r.URL.Query().Get("filter")
	query := BenchSvcBuildQuery("users", BenchSvcTransform(filter))
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
