package testcode

import (
	"fmt"
	"net/http"

	"github.com/labstack/echo/v4"
)

func BenchmarkTest00298(ctx echo.Context) error {
	id := ctx.QueryParam("id")
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", id)
	rows, err := DB.Query(query)
	if err != nil {
		return ctx.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}
	defer rows.Close()
	return ctx.JSON(http.StatusOK, map[string]string{"status": "ok"})
}
