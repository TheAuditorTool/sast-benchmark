package testcode

import (
	"net/http"
	"os"
	"path/filepath"

	"github.com/labstack/echo/v4"
)

func BenchmarkTest00302(ctx echo.Context) error {
	filename := ctx.Param("file")
	safeName := filepath.Base(filename)
	data, err := os.ReadFile("/uploads/" + safeName)
	if err != nil {
		return ctx.JSON(http.StatusNotFound, map[string]string{"error": "file not found"})
	}
	return ctx.Blob(http.StatusOK, "application/octet-stream", data)
}
