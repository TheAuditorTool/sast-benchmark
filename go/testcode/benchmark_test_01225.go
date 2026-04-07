package testcode

import (
	"net/http"
	"strconv"
	html_template "html/template"
)

var benchmarkTest01225Templates = []*html_template.Template{
	html_template.Must(html_template.New("0").Parse("<p>Page A: {{.}}</p>")),
	html_template.Must(html_template.New("1").Parse("<p>Page B: {{.}}</p>")),
	html_template.Must(html_template.New("2").Parse("<p>Page C: {{.}}</p>")),
}

func BenchmarkTest01225(w http.ResponseWriter, r *http.Request) {
	idx, err := strconv.Atoi(r.URL.Query().Get("idx"))
	if err != nil || idx < 0 || idx >= len(benchmarkTest01225Templates) {
		http.Error(w, "invalid index", http.StatusBadRequest)
		return
	}
	benchmarkTest01225Templates[idx].Execute(w, r.URL.Query().Get("data"))
}
