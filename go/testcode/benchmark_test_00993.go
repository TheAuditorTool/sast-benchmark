package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
)

func BenchmarkTest00993(w http.ResponseWriter, r *http.Request) {
	field := r.URL.Query().Get("field")
	value := r.URL.Query().Get("value")
	pipeline := mongo.Pipeline{
		bson.D{{Key: "$match", Value: bson.M{field: value}}},
	}
	cursor, err := MongoCollection.Aggregate(context.Background(), pipeline)
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer cursor.Close(context.Background())
	var results []bson.M
	cursor.All(context.Background(), &results)
	RespondJSON(w, http.StatusOK, results)
}
