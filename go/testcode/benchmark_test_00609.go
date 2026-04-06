package testcode

import (
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

type benchmarkTest00609Query struct {
	Username string `json:"username"`
	Status   string `json:"status"`
}

func BenchmarkTest00609(w http.ResponseWriter, r *http.Request) {
	var query benchmarkTest00609Query
	if err := ParseJSONBody(r, &query); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	filter := bson.M{
		"username": query.Username,
		"status":   query.Status,
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
