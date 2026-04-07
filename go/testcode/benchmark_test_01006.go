package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest01006(w http.ResponseWriter, r *http.Request) {
	var filter bson.M
	if err := ParseJSONBody(r, &filter); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}
	cursor, err := MongoCollection.Find(context.Background(), filter)
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer cursor.Close(context.Background())
	var results []bson.M
	cursor.All(context.Background(), &results)
	RespondJSON(w, http.StatusOK, results)
}
