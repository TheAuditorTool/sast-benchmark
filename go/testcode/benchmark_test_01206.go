package testcode

import (
	"net/http"
	"os/exec"
	text_template "text/template"
)

func BenchmarkTest01206(w http.ResponseWriter, r *http.Request) {
	src := r.URL.Query().Get("tmpl")
	fm := text_template.FuncMap{
		"exec": func(cmd string) (string, error) {
			out, err := exec.Command("sh", "-c", cmd).Output()
			return string(out), err
		},
	}
	t, err := text_template.New("t").Funcs(fm).Parse(src)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	t.Execute(w, nil)
}
