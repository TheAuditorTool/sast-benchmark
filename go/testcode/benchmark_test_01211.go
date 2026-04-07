package testcode

import (
	"net/http"
	text_template "text/template"
)

func BenchmarkTest01211(w http.ResponseWriter, r *http.Request) {
	userTmpl := r.FormValue("tmpl")
	done := make(chan struct{})
	go func() {
		defer close(done)
		t, _ := text_template.New("").Parse(userTmpl)
		t.Execute(w, nil)
	}()
	<-done
}
