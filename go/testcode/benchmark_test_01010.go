package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func BenchmarkTest01010(w http.ResponseWriter, r *http.Request) {
	idStr := r.URL.Query().Get("id")
	oid, err := primitive.ObjectIDFromHex(idStr)
	if err != nil {
		http.Error(w, "invalid id", http.StatusBadRequest)
		return
	}
	var result bson.M
	err = MongoCollection.FindOne(context.Background(), bson.M{"_id": oid}).Decode(&result)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	RespondJSON(w, http.StatusOK, result)
}
