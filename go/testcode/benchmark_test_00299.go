package testcode

import (
	"fmt"
	"net/http"

	"github.com/labstack/echo/v4"
)

func BenchmarkTest00299(ctx echo.Context) error {
	name := ctx.Param("name")
	query := fmt.Sprintf("SELECT * FROM users WHERE name = '%s'", name)
	var displayName string
	err := DB.QueryRow(query).Scan(&displayName)
	if err != nil {
		return ctx.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}
	return ctx.JSON(http.StatusOK, map[string]string{"name": displayName})
}
