package testcode

import (
	"net/http"
)

type benchmarkTest01256Work struct {
	key string
	val int
	rep chan int
}

var benchmarkTest01256Worker = func() chan benchmarkTest01256Work {
	ch := make(chan benchmarkTest01256Work, 32)
	go func() {
		owned := map[string]int{}
		for w := range ch {
			owned[w.key] += w.val
			w.rep <- owned[w.key]
		}
	}()
	return ch
}()

func BenchmarkTest01256(w http.ResponseWriter, r *http.Request) {
	rep := make(chan int, 1)
	benchmarkTest01256Worker <- benchmarkTest01256Work{key: "hits", val: 1, rep: rep}
	total := <-rep
	RespondJSON(w, http.StatusOK, map[string]int{"total": total})
}
