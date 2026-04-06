package testcode

import (
	"image"
	_ "image/gif"
	_ "image/jpeg"
	"image/png"
	"net/http"
	"os"
	"path/filepath"

	"github.com/google/uuid"
)

func BenchmarkTest00635(w http.ResponseWriter, r *http.Request) {
	file, _, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, "no file uploaded", http.StatusBadRequest)
		return
	}
	defer file.Close()

	img, _, err := image.Decode(file)
	if err != nil {
		http.Error(w, "invalid image file", http.StatusBadRequest)
		return
	}

	newName := uuid.New().String() + ".png"
	destPath := filepath.Join("/uploads", newName)

	dst, err := os.Create(destPath)
	if err != nil {
		http.Error(w, "failed to save file", http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	if err := png.Encode(dst, img); err != nil {
		http.Error(w, "failed to encode image", http.StatusInternalServerError)
		return
	}

	bounds := img.Bounds()
	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"status":    "uploaded",
		"stored_as": newName,
		"width":     bounds.Dx(),
		"height":    bounds.Dy(),
	})
}
