package testcode

import (
	"net/http"
	html_template "html/template"
)

var benchmarkTest01218Tmpl = html_template.Must(html_template.New("t").Parse("<p>{{.Data}}</p>"))

type benchmarkTest01218Data struct {
	Data string
}

func BenchmarkTest01218(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("data")
	benchmarkTest01218Tmpl.Execute(w, benchmarkTest01218Data{Data: userInput})
}
