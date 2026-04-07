package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
)

func BenchmarkTest01004(w http.ResponseWriter, r *http.Request) {
	var stage bson.M
	if err := ParseJSONBody(r, &stage); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}
	pipeline := mongo.Pipeline{bson.D{{Key: "$match", Value: stage}}}
	cursor, err := MongoCollection.Aggregate(context.Background(), pipeline)
	if err != nil {
		http.Error(w, "aggregate error", http.StatusInternalServerError)
		return
	}
	defer cursor.Close(context.Background())
	var results []bson.M
	cursor.All(context.Background(), &results)
	RespondJSON(w, http.StatusOK, results)
}
