package testcode

import (
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
)

func BenchmarkTest00610(w http.ResponseWriter, r *http.Request) {
	department := r.URL.Query().Get("department")
	if department == "" {
		http.Error(w, "department required", http.StatusBadRequest)
		return
	}

	pipeline := mongo.Pipeline{
		bson.D{{Key: "$match", Value: bson.D{{Key: "department", Value: department}}}},
		bson.D{{Key: "$group", Value: bson.D{
			{Key: "_id", Value: "$role"},
			{Key: "count", Value: bson.D{{Key: "$sum", Value: 1}}},
		}}},
	}

	cursor, err := MongoCollection.Aggregate(r.Context(), pipeline)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer cursor.Close(r.Context())

	var results []bson.M
	if err := cursor.All(r.Context(), &results); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, results)
}
