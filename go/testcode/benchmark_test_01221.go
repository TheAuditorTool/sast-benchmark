package testcode

import (
	html_template "html/template"
	"net/http"
)

func BenchmarkTest01221(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("msg")
	t := html_template.Must(html_template.New("t").Parse("<p>{{.}}</p>"))
	t.Execute(w, userInput)
}
