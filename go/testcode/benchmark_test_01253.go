package testcode

import (
	"net/http"
)

type benchmarkTest01253Req struct {
	key string
	val string
	rep chan string
}

var benchmarkTest01253Ch = func() chan benchmarkTest01253Req {
	ch := make(chan benchmarkTest01253Req, 64)
	go func() {
		store := map[string]string{}
		for req := range ch {
			store[req.key] = req.val
			req.rep <- store[req.key]
		}
	}()
	return ch
}()

func BenchmarkTest01253(w http.ResponseWriter, r *http.Request) {
	rep := make(chan string, 1)
	benchmarkTest01253Ch <- benchmarkTest01253Req{
		key: r.URL.Query().Get("key"),
		val: r.URL.Query().Get("val"),
		rep: rep,
	}
	result := <-rep
	RespondJSON(w, http.StatusOK, map[string]string{"val": result})
}
