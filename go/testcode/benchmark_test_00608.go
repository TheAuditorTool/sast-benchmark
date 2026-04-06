package testcode

import (
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest00608(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	if username == "" {
		http.Error(w, "username required", http.StatusBadRequest)
		return
	}

	filter := bson.D{
		{Key: "username", Value: bson.D{{Key: "$eq", Value: username}}},
	}

	cursor, err := MongoCollection.Find(r.Context(), filter)
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
