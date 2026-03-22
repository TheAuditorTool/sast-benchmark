package handlers

import (
	"database/sql"
	"encoding/json"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/go-chi/chi/v5"
	"github.com/labstack/echo/v4"
	"github.com/theauditor/vulnerable-api/internal/services"
)

// MultiHopHandler demonstrates multi-hop data flows
type MultiHopHandler struct {
	dataService *services.DataService
}

// NewMultiHopHandler creates a new multi-hop handler
func NewMultiHopHandler(db *sql.DB) *MultiHopHandler {
	return &MultiHopHandler{
		dataService: services.NewDataService(db),
	}
}

// ===============================================
// GIN MULTI-HOP HANDLERS
// ===============================================

// GinProcessQuery handles a query via Gin
func (h *MultiHopHandler) GinProcessQuery(c *gin.Context) {
	userQuery := c.Query("query")

	results, err := h.dataService.ProcessUserQuery(userQuery)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, results)
}

// GinSearchWithFilters handles filtered search via Gin
func (h *MultiHopHandler) GinSearchWithFilters(c *gin.Context) {
	searchTerm := c.Query("q")
	category := c.Query("category")
	sortBy := c.Query("sort_by")
	sortOrder := c.Query("sort_order")
	limit := 100 // Fixed for now

	results, err := h.dataService.SearchWithFilters(searchTerm, category, sortBy, sortOrder, limit)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, results)
}

// GinUpdateDynamic handles dynamic field updates via Gin
func (h *MultiHopHandler) GinUpdateDynamic(c *gin.Context) {
	tableName := c.Param("table")
	fieldName := c.PostForm("field")
	fieldValue := c.PostForm("value")
	whereClause := c.PostForm("where")

	err := h.dataService.UpdateDynamicField(tableName, fieldName, fieldValue, whereClause)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"status": "updated"})
}

// GinProcessComplex handles a 3-hop query via Gin
func (h *MultiHopHandler) GinProcessComplex(c *gin.Context) {
	userInput := c.Query("input")

	results, err := h.dataService.ProcessComplexQuery(userInput)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, results)
}

// GinMultipleSinks handles input through multiple operations
func (h *MultiHopHandler) GinMultipleSinks(c *gin.Context) {
	userInput := c.Query("input")

	err := h.dataService.ProcessDataWithMultipleSinks(userInput)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"status": "processed"})
}

// GinProcessStruct processes a JSON body via Gin
func (h *MultiHopHandler) GinProcessStruct(c *gin.Context) {
	var req services.ProcessRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	result, err := h.dataService.ProcessStructuredRequest(&req)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, result)
}

// GinProcessConditional handles conditional queries via Gin
func (h *MultiHopHandler) GinProcessConditional(c *gin.Context) {
	userInput := c.Query("input")
	queryType := c.Query("type")

	results, err := h.dataService.ProcessConditionally(userInput, queryType)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, results)
}

// GinProcessBatch processes a batch of inputs via Gin
func (h *MultiHopHandler) GinProcessBatch(c *gin.Context) {
	var inputs []string
	if err := c.ShouldBindJSON(&inputs); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	errors, err := h.dataService.ProcessBatch(inputs)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"errors": len(errors)})
}

// GinReturnValueTaint handles a 3-hop flow with return value
func (h *MultiHopHandler) GinReturnValueTaint(c *gin.Context) {
	userID := c.Param("user_id")

	err := h.dataService.UseTransformedData(userID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"status": "processed"})
}

// ===============================================
// ECHO MULTI-HOP HANDLERS
// ===============================================

// EchoProcessQuery handles a query via Echo
func (h *MultiHopHandler) EchoProcessQuery(ctx echo.Context) error {
	userQuery := ctx.QueryParam("query")

	results, err := h.dataService.ProcessUserQuery(userQuery)
	if err != nil {
		return ctx.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return ctx.JSON(http.StatusOK, results)
}

// EchoSearchFilters - 2-hop with multiple sources
func (h *MultiHopHandler) EchoSearchFilters(ctx echo.Context) error {
	searchTerm := ctx.QueryParam("q")
	category := ctx.QueryParam("category")
	sortBy := ctx.QueryParam("sort_by")
	sortOrder := ctx.QueryParam("sort_order")

	results, err := h.dataService.SearchWithFilters(searchTerm, category, sortBy, sortOrder, 100)
	if err != nil {
		return ctx.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return ctx.JSON(http.StatusOK, results)
}

// EchoProcessStruct - 2-hop with struct binding
func (h *MultiHopHandler) EchoProcessStruct(ctx echo.Context) error {
	var req services.ProcessRequest
	if err := ctx.Bind(&req); err != nil {
		return ctx.JSON(http.StatusBadRequest, map[string]string{"error": err.Error()})
	}

	result, err := h.dataService.ProcessStructuredRequest(&req)
	if err != nil {
		return ctx.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return ctx.JSON(http.StatusOK, result)
}

// ===============================================
// CHI MULTI-HOP HANDLERS
// ===============================================

// ChiProcessQuery handles a query via Chi
func (h *MultiHopHandler) ChiProcessQuery(w http.ResponseWriter, r *http.Request) {
	userQuery := r.URL.Query().Get("query")

	results, err := h.dataService.ProcessUserQuery(userQuery)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(results)
}

// ChiUpdateDynamic - 2-hop with URL params
func (h *MultiHopHandler) ChiUpdateDynamic(w http.ResponseWriter, r *http.Request) {
	tableName := chi.URLParam(r, "table")
	r.ParseForm()
	fieldName := r.PostFormValue("field")
	fieldValue := r.PostFormValue("value")
	whereClause := r.PostFormValue("where")

	err := h.dataService.UpdateDynamicField(tableName, fieldName, fieldValue, whereClause)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "updated"})
}

// ChiProcessComplex - 3-hop flow
func (h *MultiHopHandler) ChiProcessComplex(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("input")

	results, err := h.dataService.ProcessComplexQuery(userInput)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(results)
}

// ChiProcessStruct - 2-hop with JSON body
func (h *MultiHopHandler) ChiProcessStruct(w http.ResponseWriter, r *http.Request) {
	var req services.ProcessRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	result, err := h.dataService.ProcessStructuredRequest(&req)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(result)
}

// ===============================================
// NET/HTTP MULTI-HOP HANDLERS
// ===============================================

// NetHTTPProcessQuery - 2-hop flow: net/http -> Service -> SQL
func (h *MultiHopHandler) NetHTTPProcessQuery(w http.ResponseWriter, r *http.Request) {
	userQuery := r.URL.Query().Get("query")

	results, err := h.dataService.ProcessUserQuery(userQuery)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(results)
}

// NetHTTPMultipleSinks - 2-hop with multiple sinks
func (h *MultiHopHandler) NetHTTPMultipleSinks(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("input")

	err := h.dataService.ProcessDataWithMultipleSinks(userInput)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "processed"})
}

// NetHTTPReturnTaint - 3-hop with return value
func (h *MultiHopHandler) NetHTTPReturnTaint(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("user_id")

	err := h.dataService.UseTransformedData(userID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "processed"})
}
