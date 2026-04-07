package testcode

import (
	"net/http"
	html_template "html/template"
)

var benchmarkTest01220Templates = map[string]*html_template.Template{
	"home":    html_template.Must(html_template.New("home").Parse("<h1>Home</h1><p>{{.}}</p>")),
	"profile": html_template.Must(html_template.New("profile").Parse("<h1>Profile</h1><p>{{.}}</p>")),
}

func BenchmarkTest01220(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("page")
	t, ok := benchmarkTest01220Templates[name]
	if !ok {
		http.Error(w, "unknown page", http.StatusNotFound)
		return
	}
	t.Execute(w, r.URL.Query().Get("data"))
}
