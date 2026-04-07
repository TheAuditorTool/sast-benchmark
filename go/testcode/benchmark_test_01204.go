package testcode

import (
	"net/http"
	"plugin"
)

func BenchmarkTest01204(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("plugin")
	p, err := plugin.Open(name + ".so")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	sym, err := p.Lookup("Run")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	sym.(func())()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
