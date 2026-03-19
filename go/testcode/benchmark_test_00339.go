package testcode

import "net/http"

func BenchmarkTest00339(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("data")
	data := BenchSvcTransform(param)
	rows, err := DB.Query("SELECT * FROM logs WHERE entry = ?", data)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
