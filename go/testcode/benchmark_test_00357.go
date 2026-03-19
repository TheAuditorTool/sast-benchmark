package testcode

import (
	"net/http"
	"strconv"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest00357(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("age")
	age, err := strconv.Atoi(input)
	if err != nil {
		http.Error(w, "invalid age", http.StatusBadRequest)
		return
	}
	filter := bson.M{"age": bson.M{"$gt": age}}
	cursor, err := MongoCollection.Find(r.Context(), filter)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer cursor.Close(r.Context())
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
