package testcode

import (
	"net/http"
	text_template "text/template"
)

func BenchmarkTest01208(w http.ResponseWriter, r *http.Request) {
	header := r.FormValue("header")
	footer := r.FormValue("footer")
	src := header + "{{.}}" + footer
	t, err := text_template.New("").Parse(src)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	t.Execute(w, "hello")
}
