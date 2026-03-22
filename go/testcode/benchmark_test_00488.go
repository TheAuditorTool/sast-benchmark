package testcode

import (
	"encoding/json"
	"io"
	"net/http"
)

func BenchmarkTest00488(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}

	var data map[string]interface{}
	if err := json.Unmarshal(body, &data); err != nil {
		http.Error(w, "unmarshal error", http.StatusBadRequest)
		return
	}

	callbackName, ok := data["callback"].(string)
	if !ok {
		http.Error(w, "missing callback", http.StatusBadRequest)
		return
	}

	resultVal, _ := data["value"].(string)
	output := benchmarkTest00488Dispatch(callbackName, resultVal)
	RespondJSON(w, http.StatusOK, map[string]string{"result": output})
}

var benchmarkTest00488Callbacks = map[string]func(string) string{
	"upper": func(s string) string { return s },
	"lower": func(s string) string { return s },
}

func benchmarkTest00488Dispatch(name, val string) string {
	fn, ok := benchmarkTest00488Callbacks[name]
	if !ok {
		return "unknown callback"
	}
	return fn(val)
}
