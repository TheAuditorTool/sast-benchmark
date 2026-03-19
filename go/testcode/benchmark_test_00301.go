package testcode

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func BenchmarkTest00301(ctx echo.Context) error {
	id := ctx.QueryParam("id")
	rows, err := DB.Query("SELECT * FROM users WHERE id = ?", id)
	if err != nil {
		return ctx.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}
	defer rows.Close()
	return ctx.JSON(http.StatusOK, map[string]string{"status": "ok"})
}
