package testcode

import (
	"bytes"
	"net/http"
	"text/template"
)

func BenchmarkTest00384(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	tmpl := template.Must(template.New("sql").Parse("SELECT * FROM users WHERE active = 1"))
	var buf bytes.Buffer
	tmpl.Execute(&buf, nil)
	rows, err := DB.Query(buf.String()+" AND id = ?", id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
