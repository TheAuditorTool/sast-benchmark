package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func BenchmarkTest01015(w http.ResponseWriter, r *http.Request) {
	rawID := r.URL.Query().Get("doc_id")
	docID, err := primitive.ObjectIDFromHex(rawID)
	if err != nil {
		http.Error(w, "invalid document id", http.StatusBadRequest)
		return
	}
	cursor, err := MongoCollection.Find(context.Background(), bson.M{"_id": docID})
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer cursor.Close(context.Background())
	var results []bson.M
	cursor.All(context.Background(), &results)
	RespondJSON(w, http.StatusOK, results)
}
