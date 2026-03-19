package testcode

import "net/http"

func BenchmarkTest00329(w http.ResponseWriter, r *http.Request) {
	where := r.URL.Query().Get("where")
	query := BenchSvcBuildQuery("users", where)
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
