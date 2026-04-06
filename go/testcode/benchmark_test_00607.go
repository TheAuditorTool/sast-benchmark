package testcode

import (
	"encoding/json"
	"io"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest00607(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}

	var filter bson.M
	if err := json.Unmarshal(body, &filter); err != nil {
		http.Error(w, "invalid JSON", http.StatusBadRequest)
		return
	}

	result, err := MongoCollection.DeleteMany(r.Context(), filter)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"deleted": result.DeletedCount,
	})
}
