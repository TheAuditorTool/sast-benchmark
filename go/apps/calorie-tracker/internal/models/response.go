package models

// APIResponse is the standard API response wrapper
type APIResponse struct {
	Success bool        `json:"success"`
	Data    interface{} `json:"data,omitempty"`
	Error   string      `json:"error,omitempty"`
	Errors  []string    `json:"errors,omitempty"`
}

// NewSuccessResponse creates a success response
func NewSuccessResponse(data interface{}) APIResponse {
	return APIResponse{
		Success: true,
		Data:    data,
	}
}

// NewErrorResponse creates an error response
func NewErrorResponse(message string) APIResponse {
	return APIResponse{
		Success: false,
		Error:   message,
	}
}

// NewValidationErrorResponse creates a validation error response
func NewValidationErrorResponse(errors []string) APIResponse {
	return APIResponse{
		Success: false,
		Error:   "Validation failed",
		Errors:  errors,
	}
}

// PaginationParams for paginated requests
// TAINT SOURCE: Query parameters flow to SQL
type PaginationParams struct {
	Page      int    `form:"page" validate:"min=1"`
	PerPage   int    `form:"per_page" validate:"min=1,max=100"`
	SortBy    string `form:"sort_by"`
	SortOrder string `form:"sort_order" validate:"omitempty,oneof=asc desc ASC DESC"`
}

// GetOffset calculates SQL offset
func (p *PaginationParams) GetOffset() int {
	if p.Page < 1 {
		p.Page = 1
	}
	if p.PerPage < 1 {
		p.PerPage = 20
	}
	return (p.Page - 1) * p.PerPage
}

// GetLimit returns the limit
func (p *PaginationParams) GetLimit() int {
	if p.PerPage < 1 {
		return 20
	}
	if p.PerPage > 100 {
		return 100
	}
	return p.PerPage
}

// BuildOrderBy builds ORDER BY clause - VULNERABLE to SQL injection
// TAINT PROPAGATION: User input -> SQL fragment
func (p *PaginationParams) BuildOrderBy(defaultSort string) string {
	sortBy := p.SortBy
	if sortBy == "" {
		sortBy = defaultSort
	}
	sortOrder := p.SortOrder
	if sortOrder == "" {
		sortOrder = "DESC"
	}
	// VULNERABLE: Direct string concatenation
	return sortBy + " " + sortOrder
}

// DateRangeFilter for date-based queries
type DateRangeFilter struct {
	StartDate string `form:"start_date"`
	EndDate   string `form:"end_date"`
}
