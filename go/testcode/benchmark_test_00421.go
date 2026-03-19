package testcode

import (
	"bytes"
	"html/template"
	"net/http"
)

func BenchmarkTest00421(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("content")
	funcMap := template.FuncMap{
		"raw": func(s string) template.HTML { return template.HTML(s) },
	}
	tmpl := template.Must(template.New("page").Funcs(funcMap).Parse(`<div>{{raw .Content}}</div>`))
	var buf bytes.Buffer
	tmpl.Execute(&buf, map[string]string{"Content": input})
	w.Header().Set("Content-Type", "text/html")
	w.Write(buf.Bytes())
}
