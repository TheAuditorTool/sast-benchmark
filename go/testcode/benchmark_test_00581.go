package testcode

import (
	"bytes"
	"encoding/base64"
	"encoding/gob"
	"net/http"
)

func BenchmarkTest00581(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("prefs")
	if err != nil {
		http.Error(w, "missing prefs cookie", http.StatusBadRequest)
		return
	}

	decoded, err := base64.StdEncoding.DecodeString(cookie.Value)
	if err != nil {
		http.Error(w, "invalid encoding", http.StatusBadRequest)
		return
	}

	var prefs interface{}
	dec := gob.NewDecoder(bytes.NewReader(decoded))
	if err := dec.Decode(&prefs); err != nil {
		http.Error(w, "decode error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, prefs)
}
