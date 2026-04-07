package testcode

import (
	"net/http"

	"github.com/google/uuid"
)

func BenchmarkTest00773(w http.ResponseWriter, r *http.Request) {
	requestID := uuid.New().String()
	RespondJSON(w, http.StatusOK, map[string]string{"request_id": requestID})
}
