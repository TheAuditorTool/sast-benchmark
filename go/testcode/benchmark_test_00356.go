package testcode

import (
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func BenchmarkTest00356(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("id")
	id, err := primitive.ObjectIDFromHex(input)
	if err != nil {
		http.Error(w, "invalid id", http.StatusBadRequest)
		return
	}
	filter := bson.M{"_id": id}
	cursor, err := MongoCollection.Find(r.Context(), filter)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer cursor.Close(r.Context())
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
