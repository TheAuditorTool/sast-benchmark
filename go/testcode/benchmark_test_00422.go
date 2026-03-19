package testcode

import (
	"bytes"
	"html/template"
	"net/http"
)

func BenchmarkTest00422(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("content")
	tmpl := template.Must(template.New("page").Parse(`<div>{{.Content}}</div>`))
	var buf bytes.Buffer
	tmpl.Execute(&buf, map[string]string{"Content": input})
	w.Header().Set("Content-Type", "text/html")
	w.Write(buf.Bytes())
}
