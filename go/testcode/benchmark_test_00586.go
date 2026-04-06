package testcode

import (
	"encoding/gob"
	"net/http"
	"os"
)

type benchmarkTest00586Prefs struct {
	Theme    string
	PageSize int
	Locale   string
}

func BenchmarkTest00586(w http.ResponseWriter, r *http.Request) {
	f, err := os.Open("/var/app/cache/user_prefs.gob")
	if err != nil {
		http.Error(w, "prefs unavailable", http.StatusInternalServerError)
		return
	}
	defer f.Close()

	var prefs benchmarkTest00586Prefs
	dec := gob.NewDecoder(f)
	if err := dec.Decode(&prefs); err != nil {
		http.Error(w, "decode error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, prefs)
}
