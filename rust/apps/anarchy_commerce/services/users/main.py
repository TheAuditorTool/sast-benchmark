"""
Users Service (Python/FastAPI)

Handles user registration, authentication, and profile management.
Called by Gateway service.

TAINT FLOWS:
- /register: email, password, name → database
- /login: credentials → JWT generation
- /users/{id}: userId → database query (IDOR potential)
"""

from fastapi import FastAPI, HTTPException, Depends, Header
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel, EmailStr
from typing import Optional
import hashlib
import jwt
import os

app = FastAPI(title="Users Service", version="0.1.0")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:4000"],  # Only gateway
    allow_methods=["*"],
    allow_headers=["*"],
)

# Config
JWT_SECRET = os.getenv("JWT_SECRET", "dev-secret-change-in-prod")  # VULN: Weak default
DATABASE_URL = os.getenv("DATABASE_URL", "postgresql://localhost/anarchy")

# Models
class UserRegister(BaseModel):
    email: EmailStr
    password: str
    name: str

class UserLogin(BaseModel):
    email: EmailStr
    password: str

class UserUpdate(BaseModel):
    name: Optional[str] = None
    bio: Optional[str] = None
    avatar_url: Optional[str] = None

class UserResponse(BaseModel):
    id: str
    email: str
    name: str
    bio: Optional[str] = None
    avatar_url: Optional[str] = None

# Simulated database
users_db = {}

def hash_password(password: str) -> str:
    # VULN: Using MD5 for password hashing (should use bcrypt)
    return hashlib.md5(password.encode()).hexdigest()

def verify_token(authorization: str = Header(None)) -> dict:
    if not authorization or not authorization.startswith("Bearer "):
        raise HTTPException(status_code=401, detail="Missing token")

    token = authorization.replace("Bearer ", "")
    try:
        return jwt.decode(token, JWT_SECRET, algorithms=["HS256"])
    except jwt.InvalidTokenError:
        raise HTTPException(status_code=401, detail="Invalid token")


@app.post("/register", response_model=UserResponse)
async def register(user: UserRegister):
    """
    Register a new user.
    TAINT: user.email, user.password, user.name flow to database
    """
    # Check if email exists
    if user.email in users_db:
        raise HTTPException(status_code=400, detail="Email already registered")

    # VULN: Weak password hashing
    password_hash = hash_password(user.password)

    user_id = f"user_{len(users_db) + 1}"
    users_db[user.email] = {
        "id": user_id,
        "email": user.email,
        "name": user.name,
        "password_hash": password_hash,
        "bio": None,
        "avatar_url": None,
    }

    return UserResponse(**users_db[user.email])


@app.post("/login")
async def login(credentials: UserLogin):
    """
    Authenticate user and return JWT.
    TAINT: credentials flow to authentication logic
    """
    user = users_db.get(credentials.email)
    if not user:
        # VULN: Timing attack - different response for non-existent user
        raise HTTPException(status_code=401, detail="User not found")

    if user["password_hash"] != hash_password(credentials.password):
        raise HTTPException(status_code=401, detail="Invalid password")

    # Generate JWT
    token = jwt.encode(
        {"user_id": user["id"], "email": user["email"]},
        JWT_SECRET,
        algorithm="HS256"
    )

    return {"token": token, "user": UserResponse(**user)}


@app.get("/users/{user_id}", response_model=UserResponse)
async def get_user(user_id: str, token_data: dict = Depends(verify_token)):
    """
    Get user profile.
    TAINT: user_id flows to database query
    VULN: No authorization check - any authenticated user can view any profile (IDOR)
    """
    # Find user by ID
    for user in users_db.values():
        if user["id"] == user_id:
            return UserResponse(**user)

    raise HTTPException(status_code=404, detail="User not found")


@app.put("/users/{user_id}", response_model=UserResponse)
async def update_user(user_id: str, updates: UserUpdate, token_data: dict = Depends(verify_token)):
    """
    Update user profile.
    TAINT: updates.bio could contain XSS payload (stored XSS)
    VULN: No authorization check - any user can update any profile (IDOR)
    """
    # Find and update user
    for email, user in users_db.items():
        if user["id"] == user_id:
            if updates.name:
                user["name"] = updates.name
            if updates.bio:
                # VULN: No sanitization - stored XSS
                user["bio"] = updates.bio
            if updates.avatar_url:
                # VULN: No URL validation - could be javascript: URL
                user["avatar_url"] = updates.avatar_url

            return UserResponse(**user)

    raise HTTPException(status_code=404, detail="User not found")


@app.get("/health")
async def health():
    return {"status": "ok", "service": "users"}


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=4001)
