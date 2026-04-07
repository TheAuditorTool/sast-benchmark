package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest01008(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	filter := bson.M{"username": username}
	var result bson.M
	err := MongoCollection.FindOne(context.Background(), filter).Decode(&result)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	RespondJSON(w, http.StatusOK, result)
}
