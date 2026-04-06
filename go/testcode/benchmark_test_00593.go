package testcode

import (
	"html/template"
	"net/http"
)

type benchmarkTest00593Data struct {
	Name string
}

var benchmarkTest00593Tmpl = template.Must(template.New("greeting").Parse("<h1>Hello, {{.Name}}</h1>"))

func BenchmarkTest00593(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("name")
	if name == "" {
		name = "Guest"
	}

	data := benchmarkTest00593Data{Name: name}

	w.Header().Set("Content-Type", "text/html; charset=utf-8")
	if err := benchmarkTest00593Tmpl.Execute(w, data); err != nil {
		http.Error(w, "render error", http.StatusInternalServerError)
	}
}
