package testcode

import (
	"bytes"
	"net/http"
	"text/template"
)

func BenchmarkTest00383(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("name")
	table := r.URL.Query().Get("table")
	data := map[string]string{"Table": table, "Name": name}
	tmpl := template.Must(template.New("sql").Parse("SELECT * FROM {{.Table}} WHERE name = '{{.Name}}'"))
	var buf bytes.Buffer
	tmpl.Execute(&buf, data)
	rows, err := DB.Query(buf.String())
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
