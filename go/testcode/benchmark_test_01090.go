package testcode

import (
	"fmt"
	"net/http"
)

var benchmarkTest01090PrivKeyPEM = `-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEA2a2rwplBQLF29amygykEMmYz0+Kcj3bKBp29pNmDCzEqUpFJ
-----END RSA PRIVATE KEY-----`

func BenchmarkTest01090(w http.ResponseWriter, r *http.Request) {
	RespondJSON(w, http.StatusOK, map[string]string{"key_len": fmt.Sprintf("%d", len(benchmarkTest01090PrivKeyPEM))})
}
