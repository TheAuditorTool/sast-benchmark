package testcode

import (
	html_template "html/template"
	"net/http"
	"sync"
)

var benchmarkTest01231Pool = &sync.Pool{
	New: func() interface{} {
		return html_template.Must(html_template.New("t").Parse("<div>{{.Message}}</div>"))
	},
}

type benchmarkTest01231Payload struct{ Message string }

func BenchmarkTest01231(w http.ResponseWriter, r *http.Request) {
	t := benchmarkTest01231Pool.Get().(*html_template.Template)
	defer benchmarkTest01231Pool.Put(t)
	msg := r.URL.Query().Get("msg")
	t.Execute(w, benchmarkTest01231Payload{Message: msg})
}
