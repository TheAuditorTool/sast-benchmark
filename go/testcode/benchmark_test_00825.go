package testcode

import (
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00825(w http.ResponseWriter, r *http.Request) {
	data := []byte(r.FormValue("data"))
	key := byte(0x5A)
	ct := make([]byte, len(data))
	for i, b := range data {
		ct[i] = b ^ key
	}
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
