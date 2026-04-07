package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
)

func BenchmarkTest00759(w http.ResponseWriter, r *http.Request) {
	tokenCh := make(chan string, 1)
	go func() {
		tokenCh <- fmt.Sprintf("%x%x", rand.Int63(), rand.Int63())
	}()
	token := <-tokenCh
	RespondJSON(w, http.StatusOK, map[string]string{"invite_code": token})
}
