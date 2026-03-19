package testcode

import (
	"bytes"
	"net/http"
)

func BenchmarkTest00289(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("id")
	var buf bytes.Buffer
	buf.WriteString("SELECT * FROM users WHERE id = '")
	buf.WriteString(param)
	buf.WriteString("'")
	rows, err := DB.Query(buf.String())
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
