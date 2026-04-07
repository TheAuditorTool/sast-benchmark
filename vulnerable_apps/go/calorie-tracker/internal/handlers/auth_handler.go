package handlers

import (
	"errors"
	"net/http"

	"github.com/gin-gonic/gin"

	"github.com/example/calorie-tracker/internal/middleware"
	"github.com/example/calorie-tracker/internal/models"
	"github.com/example/calorie-tracker/internal/services"
)

// AuthHandler handles authentication endpoints
type AuthHandler struct {
	userService *services.UserService
}

// NewAuthHandler creates a new auth handler
func NewAuthHandler(userService *services.UserService) *AuthHandler {
	return &AuthHandler{userService: userService}
}

// Register handles user registration
// POST /api/auth/register
func (h *AuthHandler) Register(c *gin.Context) {
	var req models.CreateUserRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	user, err := h.userService.Register(req)
	if err != nil {
		if errors.Is(err, services.ErrEmailExists) {
			c.JSON(http.StatusConflict, models.NewErrorResponse("Email already exists"))
			return
		}
		if errors.Is(err, services.ErrUsernameExists) {
			c.JSON(http.StatusConflict, models.NewErrorResponse("Username already exists"))
			return
		}
		if validationErr, ok := err.(*services.ValidationError); ok {
			c.JSON(http.StatusBadRequest, models.NewValidationErrorResponse(validationErr.Errors))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to create user"))
		return
	}

	c.JSON(http.StatusCreated, models.NewSuccessResponse(user.ToProfile()))
}

// Login handles user login
// POST /api/auth/login
func (h *AuthHandler) Login(c *gin.Context) {
	var req models.LoginRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	session, err := h.userService.Login(req)
	if err != nil {
		if errors.Is(err, services.ErrInvalidCredentials) {
			c.JSON(http.StatusUnauthorized, models.NewErrorResponse("Invalid email or password"))
			return
		}
		if validationErr, ok := err.(*services.ValidationError); ok {
			c.JSON(http.StatusBadRequest, models.NewValidationErrorResponse(validationErr.Errors))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Login failed"))
		return
	}

	// Set session
	if err := middleware.SetSession(c, session.UserID, session.Username, session.Email); err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to create session"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(session))
}

// Logout handles user logout
// POST /api/auth/logout
func (h *AuthHandler) Logout(c *gin.Context) {
	if err := middleware.ClearSession(c); err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to logout"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(gin.H{"message": "Logged out"}))
}

// GetProfile gets the current user's profile
// GET /api/auth/profile
func (h *AuthHandler) GetProfile(c *gin.Context) {
	userID := middleware.GetUserID(c)

	user, err := h.userService.FindByID(userID)
	if err != nil {
		c.JSON(http.StatusNotFound, models.NewErrorResponse("User not found"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(user.ToProfile()))
}

// UpdateProfile updates the current user's profile
// PUT /api/auth/profile
func (h *AuthHandler) UpdateProfile(c *gin.Context) {
	userID := middleware.GetUserID(c)

	var req models.UpdateUserRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	user, err := h.userService.Update(userID, req)
	if err != nil {
		if errors.Is(err, services.ErrUsernameExists) {
			c.JSON(http.StatusConflict, models.NewErrorResponse("Username already exists"))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to update profile"))
		return
	}

	// Update session if username changed
	if req.Username != nil {
		middleware.SetSession(c, user.ID, user.Username, user.Email)
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(user.ToProfile()))
}

// LoginPage renders the login page
// GET /login
func (h *AuthHandler) LoginPage(c *gin.Context) {
	c.HTML(http.StatusOK, "login.html", gin.H{})
}

// RegisterPage renders the registration page
// GET /register
func (h *AuthHandler) RegisterPage(c *gin.Context) {
	c.HTML(http.StatusOK, "register.html", gin.H{})
}
