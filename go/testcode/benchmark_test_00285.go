package testcode

import (
	"net/http"
)

type benchmarkTest00285QueryReq struct {
	Query string
}

func BenchmarkTest00285(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("q")
	req := benchmarkTest00285QueryReq{Query: param}
	rows, err := DB.Query("SELECT * FROM data WHERE filter = '" + req.Query + "'")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
