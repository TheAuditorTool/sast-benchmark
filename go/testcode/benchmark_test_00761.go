package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
)

type benchmarkTest00761TokenGenerator interface {
	Generate() string
}

type benchmarkTest00761MathRandGen struct{}

func (g benchmarkTest00761MathRandGen) Generate() string {
	return fmt.Sprintf("%d", rand.Int())
}

var benchmarkTest00761Generator benchmarkTest00761TokenGenerator = benchmarkTest00761MathRandGen{}

func BenchmarkTest00761(w http.ResponseWriter, r *http.Request) {
	token := benchmarkTest00761Generator.Generate()
	RespondJSON(w, http.StatusOK, map[string]string{"csrf_token": token})
}
