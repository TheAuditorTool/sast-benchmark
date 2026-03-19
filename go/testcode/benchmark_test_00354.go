package testcode

import (
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest00354(w http.ResponseWriter, r *http.Request) {
	excludeRole := r.URL.Query().Get("exclude_role")
	filter := bson.M{"role": bson.M{"$ne": excludeRole}}
	cursor, err := MongoCollection.Find(r.Context(), filter)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer cursor.Close(r.Context())
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
