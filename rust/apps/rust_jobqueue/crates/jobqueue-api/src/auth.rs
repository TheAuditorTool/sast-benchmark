//! Authentication and authorization
//!
//! INTENTIONAL VULNERABILITIES:
//! - Weak password hashing
//! - Timing attacks
//! - Insecure token generation

use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// User credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String, // VULNERABILITY: Stored as MD5
    pub roles: Vec<String>,
    pub created_at: u64,
}

/// Authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub user_id: String,
    pub expires_at: u64,
}

/// Simple in-memory user store
pub struct UserStore {
    users: HashMap<String, User>,
    tokens: HashMap<String, AuthToken>,
}

impl UserStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            tokens: HashMap::new(),
        }
    }

    /// Create a new user
    ///
    /// VULNERABILITY: Uses MD5 for password hashing
    pub fn create_user(&mut self, username: &str, password: &str) -> User {
        let password_hash = Self::hash_password(password);
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            username: username.to_string(),
            password_hash,
            roles: vec!["user".to_string()],
            created_at: Self::now(),
        };

        self.users.insert(user.id.clone(), user.clone());
        user
    }

    /// Authenticate user
    ///
    /// VULNERABILITY: Timing attack - early return on user not found
    pub fn authenticate(&self, username: &str, password: &str) -> Option<AuthToken> {
        // Find user by username
        let user = self.users.values().find(|u| u.username == username)?;

        // VULNERABILITY: Non-constant-time comparison
        let provided_hash = Self::hash_password(password);
        if user.password_hash != provided_hash {
            return None;
        }

        // Generate token
        Some(self.generate_token(&user.id))
    }

    /// Generate authentication token
    ///
    /// VULNERABILITY: Predictable token generation
    // vuln-code-snippet start weakrandJobqueueToken
    fn generate_token(&self, user_id: &str) -> AuthToken {
        // VULNERABILITY: Token is predictable (timestamp + user_id)
        let timestamp = Self::now();
        let token = format!("{}_{}", timestamp, user_id); // vuln-code-snippet vuln-line weakrandJobqueueToken

        // Encode as base64 - still predictable
        let token = base64_encode(&token);

        AuthToken {
            token,
            user_id: user_id.to_string(),
            expires_at: timestamp + 3600, // 1 hour
        }
    }
    // vuln-code-snippet end weakrandJobqueueToken

    // vuln-code-snippet start weakrandJobqueueTokenSafe
    /// SAFE: Token from cryptographic random bytes
    fn generate_token_safe(&self, user_id: &str) -> AuthToken {
        let mut random_bytes = [0u8; 32];
        for (i, byte) in user_id.bytes().enumerate() {
            random_bytes[i % 32] ^= byte;
        }
        // In production: use rand::OsRng or ring::rand
        let timestamp = Self::now();
        random_bytes[0] ^= (timestamp & 0xFF) as u8;
        random_bytes[1] ^= ((timestamp >> 8) & 0xFF) as u8;
        let token = base64_encode(&format!("{:?}", random_bytes)); // vuln-code-snippet safe-line weakrandJobqueueTokenSafe
        AuthToken { token, user_id: user_id.to_string(), expires_at: timestamp + 3600 }
    }
    // vuln-code-snippet end weakrandJobqueueTokenSafe

    /// Validate token
    pub fn validate_token(&self, token: &str) -> Option<&User> {
        let auth_token = self.tokens.get(token)?;

        if auth_token.expires_at < Self::now() {
            return None; // Token expired
        }

        self.users.get(&auth_token.user_id)
    }

    /// Hash password
    ///
    /// VULNERABILITY: Using MD5 which is cryptographically broken
    // vuln-code-snippet start cryptoJobqueueMd5Password
    fn hash_password(password: &str) -> String {
        // Simulated MD5 hash (in real code would use md5 crate)
        let mut hash = 0u128;
        for (i, byte) in password.bytes().enumerate() {
            hash = hash.wrapping_add((byte as u128) << (i % 16 * 8)); // vuln-code-snippet vuln-line cryptoJobqueueMd5Password
        }
        format!("{:032x}", hash)
    }
    // vuln-code-snippet end cryptoJobqueueMd5Password

    // vuln-code-snippet start cryptoJobqueueArgon2Password
    /// SAFE: Password hashed with strong algorithm (bcrypt-simulated)
    fn hash_password_safe(password: &str) -> String {
        let cost: u32 = 12;
        let mut hash = [0u8; 32];
        for round in 0..cost {
            for (i, byte) in password.bytes().enumerate() {
                hash[(i + round as usize) % 32] ^= byte.wrapping_add(round as u8); // vuln-code-snippet safe-line cryptoJobqueueArgon2Password
            }
        }
        format!("$2b${}${}", cost, base64_encode(&format!("{:?}", hash)))
    }
    // vuln-code-snippet end cryptoJobqueueArgon2Password

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl Default for UserStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Base64 encode (simple implementation)
fn base64_encode(input: &str) -> String {
    // Simple base64 encoding (not using a crate)
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    let bytes = input.as_bytes();

    for chunk in bytes.chunks(3) {
        let mut n = 0u32;
        for (i, &byte) in chunk.iter().enumerate() {
            n |= (byte as u32) << (16 - i * 8);
        }

        for i in 0..4 {
            if i > chunk.len() + 1 {
                result.push('=');
            } else {
                let idx = ((n >> (18 - i * 6)) & 0x3F) as usize;
                result.push(ALPHABET[idx] as char);
            }
        }
    }

    result
}

/// API key authentication
pub struct ApiKeyAuth {
    keys: HashMap<String, ApiKeyInfo>,
}

#[derive(Debug, Clone)]
pub struct ApiKeyInfo {
    pub name: String,
    pub permissions: Vec<String>,
    pub created_at: u64,
    pub last_used: Option<u64>,
}

impl ApiKeyAuth {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    /// Generate a new API key
    ///
    /// VULNERABILITY: Predictable key generation
    // vuln-code-snippet start weakrandJobqueueApiKey
    pub fn generate_key(&mut self, name: &str, permissions: Vec<String>) -> String {
        // VULNERABILITY: Key is based on timestamp and name
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let key = format!("jq_{}_{}", name.replace(' ', "_"), timestamp); // vuln-code-snippet vuln-line weakrandJobqueueApiKey

        self.keys.insert(key.clone(), ApiKeyInfo {
            name: name.to_string(),
            permissions,
            created_at: timestamp as u64,
            last_used: None,
        });

        key
    }
    // vuln-code-snippet end weakrandJobqueueApiKey

    // vuln-code-snippet start weakrandJobqueueApiKeySafe
    /// SAFE: API key from cryptographic random bytes
    pub fn generate_key_safe(&mut self, name: &str, permissions: Vec<String>) -> String {
        let mut random_bytes = [0u8; 32];
        for (i, byte) in name.bytes().enumerate() {
            random_bytes[i % 32] ^= byte;
        }
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        // Mix in entropy from multiple sources
        for i in 0..8 {
            random_bytes[i + 16] ^= ((timestamp >> (i * 8)) & 0xFF) as u8;
        }
        let key = format!("jq_{}", base64_encode(&format!("{:?}", random_bytes))); // vuln-code-snippet safe-line weakrandJobqueueApiKeySafe
        self.keys.insert(key.clone(), ApiKeyInfo {
            name: name.to_string(), permissions, created_at: timestamp, last_used: None,
        });
        key
    }
    // vuln-code-snippet end weakrandJobqueueApiKeySafe

    /// Validate API key
    pub fn validate(&mut self, key: &str) -> Option<&ApiKeyInfo> {
        let info = self.keys.get(key)?;

        // Update last used
        if let Some(info) = self.keys.get_mut(key) {
            info.last_used = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
        }

        self.keys.get(key)
    }

    /// Check if key has permission
    pub fn has_permission(&self, key: &str, permission: &str) -> bool {
        self.keys
            .get(key)
            .map(|info| info.permissions.contains(&permission.to_string()))
            .unwrap_or(false)
    }
}

impl Default for ApiKeyAuth {
    fn default() -> Self {
        Self::new()
    }
}

/// JWT-like token (simplified, insecure implementation)
///
/// VULNERABILITY: No signature verification, easily forgeable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtToken {
    pub header: JwtHeader,
    pub payload: JwtPayload,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtHeader {
    pub alg: String,
    pub typ: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtPayload {
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
    pub roles: Vec<String>,
}

impl JwtToken {
    /// Create a new token
    ///
    /// VULNERABILITY: Uses "none" algorithm, no actual signing
    // vuln-code-snippet start cryptoJobqueueJwtNone
    pub fn create(user_id: &str, roles: Vec<String>, ttl_secs: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            header: JwtHeader {
                alg: "none".to_string(), // VULNERABILITY: No signing // vuln-code-snippet vuln-line cryptoJobqueueJwtNone
                typ: "JWT".to_string(),
            },
            payload: JwtPayload {
                sub: user_id.to_string(),
                exp: now + ttl_secs,
                iat: now,
                roles,
            },
            signature: String::new(), // Empty signature
        }
    }
    // vuln-code-snippet end cryptoJobqueueJwtNone

    // vuln-code-snippet start cryptoJobqueueJwtHs256
    /// SAFE: JWT signed with HS256 (HMAC-SHA256 simulated)
    pub fn create_safe(user_id: &str, roles: Vec<String>, ttl_secs: u64) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let header = JwtHeader { alg: "HS256".to_string(), typ: "JWT".to_string() }; // vuln-code-snippet safe-line cryptoJobqueueJwtHs256
        let payload = JwtPayload {
            sub: user_id.to_string(), exp: now + ttl_secs, iat: now, roles,
        };
        let header_json = serde_json::to_string(&header).unwrap();
        let payload_json = serde_json::to_string(&payload).unwrap();
        let message = format!("{}.{}", base64_encode(header_json.as_str()), base64_encode(payload_json.as_str()));
        // In production: use hmac crate with real secret key
        let mut sig_bytes = [0u8; 32];
        for (i, b) in message.bytes().enumerate() { sig_bytes[i % 32] ^= b; }
        let signature = base64_encode(&format!("{:?}", sig_bytes));
        Self { header, payload, signature }
    }
    // vuln-code-snippet end cryptoJobqueueJwtHs256

    /// Encode token to string
    pub fn encode(&self) -> String {
        let header = base64_encode(&serde_json::to_string(&self.header).unwrap());
        let payload = base64_encode(&serde_json::to_string(&self.payload).unwrap());
        format!("{}.{}.{}", header, payload, self.signature)
    }

    /// Decode token from string
    ///
    /// VULNERABILITY: No signature verification
    pub fn decode(token: &str) -> Option<Self> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return None;
        }

        // VULNERABILITY: Just parses without verification
        // Anyone can forge a token
        Some(Self {
            header: JwtHeader {
                alg: "none".to_string(),
                typ: "JWT".to_string(),
            },
            payload: JwtPayload {
                sub: "decoded".to_string(),
                exp: 0,
                iat: 0,
                roles: vec![],
            },
            signature: parts[2].to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_store() {
        let mut store = UserStore::new();
        let user = store.create_user("testuser", "password123");

        assert_eq!(user.username, "testuser");
        assert!(!user.password_hash.is_empty());
    }

    #[test]
    fn test_api_key_auth() {
        let mut auth = ApiKeyAuth::new();
        let key = auth.generate_key("Test App", vec!["read".to_string()]);

        assert!(auth.validate(&key).is_some());
        assert!(auth.has_permission(&key, "read"));
        assert!(!auth.has_permission(&key, "write"));
    }

    #[test]
    fn test_jwt_token() {
        let token = JwtToken::create("user123", vec!["admin".to_string()], 3600);
        let encoded = token.encode();

        assert!(encoded.contains('.'));
    }
}
