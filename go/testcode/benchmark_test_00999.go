package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

type benchmarkTest00999SearchFilter struct {
	Filter bson.M `json:"filter"`
}

func BenchmarkTest00999(w http.ResponseWriter, r *http.Request) {
	var req benchmarkTest00999SearchFilter
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}
	cursor, err := MongoCollection.Find(context.Background(), req.Filter)
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer cursor.Close(context.Background())
	var results []bson.M
	cursor.All(context.Background(), &results)
	RespondJSON(w, http.StatusOK, results)
}
