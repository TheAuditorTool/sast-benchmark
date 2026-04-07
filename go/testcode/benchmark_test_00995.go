package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest00995(w http.ResponseWriter, r *http.Request) {
	userKey := r.URL.Query().Get("key")
	userVal := r.URL.Query().Get("value")
	_, err := MongoCollection.DeleteMany(context.Background(), bson.M{userKey: userVal})
	if err != nil {
		http.Error(w, "delete error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "deleted"})
}
