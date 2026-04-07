<?php
// vuln_blog - Authentication
require_once __DIR__ . '/config.php';

// vuln-code-snippet start vb_sqli_login
function loginUnsafe(string $username, string $password): ?array {
    $pdo = getDbConnectionHardcoded();
    $query = "SELECT * FROM users WHERE username = '" . $username . "' AND password = '" . $password . "'"; // vuln-code-snippet vuln-line vb_sqli_login
    $result = $pdo->query($query);
    return $result->fetch(PDO::FETCH_ASSOC) ?: null;
}
// vuln-code-snippet end vb_sqli_login

// vuln-code-snippet start vb_sqli_login_safe
function loginSafe(string $username, string $password): ?array {
    $pdo = getDbConnectionEnv();
    $stmt = $pdo->prepare("SELECT * FROM users WHERE username = ? AND password_hash = ?"); // vuln-code-snippet safe-line vb_sqli_login_safe
    $stmt->execute([$username, $password]);
    return $stmt->fetch(PDO::FETCH_ASSOC) ?: null;
}
// vuln-code-snippet end vb_sqli_login_safe

// vuln-code-snippet start vb_weakhash_md5
function hashPasswordMd5(string $password): string {
    return md5($password); // vuln-code-snippet vuln-line vb_weakhash_md5
}
// vuln-code-snippet end vb_weakhash_md5

// vuln-code-snippet start vb_weakhash_bcrypt
function hashPasswordBcrypt(string $password): string {
    return password_hash($password, PASSWORD_BCRYPT); // vuln-code-snippet safe-line vb_weakhash_bcrypt
}
// vuln-code-snippet end vb_weakhash_bcrypt

// vuln-code-snippet start vb_weakrand_session
function generateSessionTokenWeak(): string {
    $token = rand(100000, 999999); // vuln-code-snippet vuln-line vb_weakrand_session
    return (string) $token;
}
// vuln-code-snippet end vb_weakrand_session

// vuln-code-snippet start vb_weakrand_safe
function generateSessionTokenSafe(): string {
    $token = bin2hex(random_bytes(32)); // vuln-code-snippet safe-line vb_weakrand_safe
    return $token;
}
// vuln-code-snippet end vb_weakrand_safe

// vuln-code-snippet start vb_tj_auth_bypass
function verifyTokenLoose(string $input, string $stored): bool {
    if ($input == $stored) { // vuln-code-snippet vuln-line vb_tj_auth_bypass
        return true;
    }
    return false;
}
// vuln-code-snippet end vb_tj_auth_bypass

// vuln-code-snippet start vb_tj_strict_auth
function verifyTokenStrict(string $input, string $stored): bool {
    if ($input === $stored) { // vuln-code-snippet safe-line vb_tj_strict_auth
        return true;
    }
    return false;
}
// vuln-code-snippet end vb_tj_strict_auth
