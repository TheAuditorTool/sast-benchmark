package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
	"time"
)

type benchmarkTest00758TokenGen struct {
	rng *rand.Rand
}

var benchmarkTest00758Gen = benchmarkTest00758TokenGen{
	rng: rand.New(rand.NewSource(time.Now().Unix())),
}

func (g *benchmarkTest00758TokenGen) Generate() string {
	return fmt.Sprintf("%x", g.rng.Int63())
}

func BenchmarkTest00758(w http.ResponseWriter, r *http.Request) {
	token := benchmarkTest00758Gen.Generate()
	RespondJSON(w, http.StatusOK, map[string]string{"api_token": token})
}
