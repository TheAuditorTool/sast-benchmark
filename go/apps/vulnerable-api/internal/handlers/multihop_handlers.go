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

// MultiHopHandler demonstrates multi-hop taint flows
// TAINT FLOW: Handler (HOP 1) -> Service (HOP 2) -> SQL Sink
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

// GinProcessQuery - 2-hop flow: Gin Handler -> Service -> SQL
// TAINT SOURCE: c.Query("query")
// TAINT FLOW: c.Query -> dataService.ProcessUserQuery -> SQL
func (h *MultiHopHandler) GinProcessQuery(c *gin.Context) {
	// HOP 1 TAINT SOURCE: Query parameter
	userQuery := c.Query("query")

	// HOP 2: Flow to service (cross-file)
	results, err := h.dataService.ProcessUserQuery(userQuery)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, results)
}

// GinSearchWithFilters - 2-hop flow with multiple sources
// TAINT SOURCES: Multiple c.Query() calls
func (h *MultiHopHandler) GinSearchWithFilters(c *gin.Context) {
	// HOP 1 TAINT SOURCES: Multiple query parameters
	searchTerm := c.Query("q")
	category := c.Query("category")
	sortBy := c.Query("sort_by")
	sortOrder := c.Query("sort_order")
	limit := 100 // Fixed for now

	// HOP 2: Flow to service (all tainted)
	results, err := h.dataService.SearchWithFilters(searchTerm, category, sortBy, sortOrder, limit)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, results)
}

// GinUpdateDynamic - 2-hop flow with dynamic table/field
// TAINT SOURCES: c.Param, c.PostForm
func (h *MultiHopHandler) GinUpdateDynamic(c *gin.Context) {
	// HOP 1 TAINT SOURCES
	tableName := c.Param("table")
	fieldName := c.PostForm("field")
	fieldValue := c.PostForm("value")
	whereClause := c.PostForm("where")

	// HOP 2: Flow to service
	err := h.dataService.UpdateDynamicField(tableName, fieldName, fieldValue, whereClause)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"status": "updated"})
}

// GinProcessComplex - 3-hop flow: Handler -> Service -> Helper
// TAINT SOURCE: c.Query("input")
func (h *MultiHopHandler) GinProcessComplex(c *gin.Context) {
	// HOP 1 TAINT SOURCE
	userInput := c.Query("input")

	// HOP 2 -> HOP 3: Service calls helper
	results, err := h.dataService.ProcessComplexQuery(userInput)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, results)
}

// GinMultipleSinks - 2-hop flow with multiple sinks
// TAINT SOURCE: c.Query("input")
func (h *MultiHopHandler) GinMultipleSinks(c *gin.Context) {
	// HOP 1 TAINT SOURCE
	userInput := c.Query("input")

	// HOP 2: Service has 4 different sinks
	err := h.dataService.ProcessDataWithMultipleSinks(userInput)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"status": "processed"})
}

// GinProcessStruct - 2-hop flow with struct taint
// TAINT SOURCE: c.ShouldBindJSON
func (h *MultiHopHandler) GinProcessStruct(c *gin.Context) {
	// HOP 1 TAINT SOURCE: JSON body -> struct
	var req services.ProcessRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	// HOP 2: Struct fields flow to service
	result, err := h.dataService.ProcessStructuredRequest(&req)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, result)
}

// GinProcessConditional - 2-hop with conditional flow
// TAINT SOURCES: c.Query("input"), c.Query("type")
func (h *MultiHopHandler) GinProcessConditional(c *gin.Context) {
	// HOP 1 TAINT SOURCES
	userInput := c.Query("input")
	queryType := c.Query("type")

	// HOP 2: Conditional taint propagation
	results, err := h.dataService.ProcessConditionally(userInput, queryType)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, results)
}

// GinProcessBatch - 2-hop with loop taint
// TAINT SOURCE: c.ShouldBindJSON (array)
func (h *MultiHopHandler) GinProcessBatch(c *gin.Context) {
	// HOP 1 TAINT SOURCE: JSON array
	var inputs []string
	if err := c.ShouldBindJSON(&inputs); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	// HOP 2: Loop taint propagation
	errors, err := h.dataService.ProcessBatch(inputs)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"errors": len(errors)})
}

// GinReturnValueTaint - 3-hop with return value taint
// TAINT SOURCE: c.Param("user_id")
func (h *MultiHopHandler) GinReturnValueTaint(c *gin.Context) {
	// HOP 1 TAINT SOURCE
	userID := c.Param("user_id")

	// HOP 2 -> HOP 3: Service returns tainted value, uses it
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

// EchoProcessQuery - 2-hop flow: Echo Handler -> Service -> SQL
// TAINT SOURCE: ctx.QueryParam("query")
func (h *MultiHopHandler) EchoProcessQuery(ctx echo.Context) error {
	// HOP 1 TAINT SOURCE
	userQuery := ctx.QueryParam("query")

	// HOP 2: Flow to service
	results, err := h.dataService.ProcessUserQuery(userQuery)
	if err != nil {
		return ctx.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return ctx.JSON(http.StatusOK, results)
}

// EchoSearchFilters - 2-hop with multiple sources
func (h *MultiHopHandler) EchoSearchFilters(ctx echo.Context) error {
	// HOP 1 TAINT SOURCES
	searchTerm := ctx.QueryParam("q")
	category := ctx.QueryParam("category")
	sortBy := ctx.QueryParam("sort_by")
	sortOrder := ctx.QueryParam("sort_order")

	// HOP 2: Flow to service
	results, err := h.dataService.SearchWithFilters(searchTerm, category, sortBy, sortOrder, 100)
	if err != nil {
		return ctx.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return ctx.JSON(http.StatusOK, results)
}

// EchoProcessStruct - 2-hop with struct binding
func (h *MultiHopHandler) EchoProcessStruct(ctx echo.Context) error {
	// HOP 1 TAINT SOURCE
	var req services.ProcessRequest
	if err := ctx.Bind(&req); err != nil {
		return ctx.JSON(http.StatusBadRequest, map[string]string{"error": err.Error()})
	}

	// HOP 2: Struct flows to service
	result, err := h.dataService.ProcessStructuredRequest(&req)
	if err != nil {
		return ctx.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return ctx.JSON(http.StatusOK, result)
}

// ===============================================
// CHI MULTI-HOP HANDLERS
// ===============================================

// ChiProcessQuery - 2-hop flow: Chi Handler -> Service -> SQL
// TAINT SOURCE: chi.URLParam, r.URL.Query()
func (h *MultiHopHandler) ChiProcessQuery(w http.ResponseWriter, r *http.Request) {
	// HOP 1 TAINT SOURCE
	userQuery := r.URL.Query().Get("query")

	// HOP 2: Flow to service
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
	// HOP 1 TAINT SOURCES
	tableName := chi.URLParam(r, "table")
	r.ParseForm()
	fieldName := r.PostFormValue("field")
	fieldValue := r.PostFormValue("value")
	whereClause := r.PostFormValue("where")

	// HOP 2: Flow to service
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
	// HOP 1 TAINT SOURCE
	userInput := r.URL.Query().Get("input")

	// HOP 2 -> HOP 3
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
	// HOP 1 TAINT SOURCE
	var req services.ProcessRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// HOP 2
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
	// HOP 1 TAINT SOURCE
	userQuery := r.URL.Query().Get("query")

	// HOP 2
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
	// HOP 1 TAINT SOURCE
	userInput := r.URL.Query().Get("input")

	// HOP 2: 4 different sinks
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
	// HOP 1 TAINT SOURCE
	userID := r.URL.Query().Get("user_id")

	// HOP 2 -> HOP 3
	err := h.dataService.UseTransformedData(userID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "processed"})
}
