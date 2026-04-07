package testcode

import (
	"net/http"
	text_template "text/template"
)

func benchmarkTest01210LoadTmpl(src string) *text_template.Template {
	t, _ := text_template.New("").Parse(src)
	return t
}

func BenchmarkTest01210(w http.ResponseWriter, r *http.Request) {
	src := r.URL.Query().Get("src")
	t := benchmarkTest01210LoadTmpl(src)
	data := map[string]string{"Host": r.Host}
	if err := t.Execute(w, data); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}
