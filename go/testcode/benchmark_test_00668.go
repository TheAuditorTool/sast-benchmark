package testcode

import (
	"bytes"
	"encoding/json"
	"net/http"
	"sync"
)

var benchmarkTest00668Pool = sync.Pool{
	New: func() interface{} {
		return new(bytes.Buffer)
	},
}

func BenchmarkTest00668(w http.ResponseWriter, r *http.Request) {
	var payload map[string]interface{}
	if err := ParseJSONBody(r, &payload); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	buf := benchmarkTest00668Pool.Get().(*bytes.Buffer)
	defer func() {
		buf.Reset()
		benchmarkTest00668Pool.Put(buf)
	}()

	enc := json.NewEncoder(buf)
	enc.SetIndent("", "  ")
	if err := enc.Encode(payload); err != nil {
		http.Error(w, "encoding error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	w.Write(buf.Bytes())
}
