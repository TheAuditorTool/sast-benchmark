package testcode

import (
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest00534(w http.ResponseWriter, r *http.Request) {
	email := r.URL.Query().Get("email")

	filter := bson.M{"email": bson.M{"$eq": email}}

	_ = filter
	RespondJSON(w, http.StatusOK, map[string]string{"status": "found"})
}
