package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

type benchmarkTest01012SearchReq struct {
	Username string `json:"username"`
	Status   string `json:"status"`
}

func BenchmarkTest01012(w http.ResponseWriter, r *http.Request) {
	var req benchmarkTest01012SearchReq
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}
	filter := bson.M{
		"username": req.Username,
		"status":   req.Status,
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
