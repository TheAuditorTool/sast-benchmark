package testcode

import (
	"net/http"
	"plugin"
)

var benchmarkTest01209Store = map[string]string{}

func BenchmarkTest01209(w http.ResponseWriter, r *http.Request) {
	if r.Method == http.MethodPost {
		sessionID := r.Header.Get("X-Session-ID")
		benchmarkTest01209Store[sessionID] = r.FormValue("plugin_name")
		RespondJSON(w, http.StatusOK, map[string]string{"status": "saved"})
		return
	}
	sessionID := r.Header.Get("X-Session-ID")
	pluginName := benchmarkTest01209Store[sessionID]
	p, err := plugin.Open(pluginName + ".so")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	sym, _ := p.Lookup("Run")
	sym.(func())()
}
