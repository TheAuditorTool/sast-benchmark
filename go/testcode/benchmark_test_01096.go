package testcode

import "net/http"

var benchmarkTest01096Obfuscated = []byte{0x73, 0x65, 0x63, 0x72, 0x65, 0x74}
var benchmarkTest01096Mask = byte(0x00)

func BenchmarkTest01096(w http.ResponseWriter, r *http.Request) {
	decoded := make([]byte, len(benchmarkTest01096Obfuscated))
	for i, b := range benchmarkTest01096Obfuscated {
		decoded[i] = b ^ benchmarkTest01096Mask
	}
	RespondJSON(w, http.StatusOK, map[string]string{"key": string(decoded)})
}
