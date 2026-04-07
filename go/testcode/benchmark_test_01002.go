package testcode

import (
	"context"
	"io"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest01002(w http.ResponseWriter, r *http.Request) {
	userBytes, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}
	cursor, err := MongoCollection.Find(context.Background(), bson.Raw(userBytes))
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer cursor.Close(context.Background())
	var results []bson.M
	cursor.All(context.Background(), &results)
	RespondJSON(w, http.StatusOK, results)
}
