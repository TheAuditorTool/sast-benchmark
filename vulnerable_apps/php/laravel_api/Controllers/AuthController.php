<?php
// laravel_api - AuthController: hardcoded secrets, weak randomness, CSRF tests
require_once __DIR__ . '/../../../testcode/shared.php';

// vuln-code-snippet start la_hardcoded_jwt
function la_hardcoded_jwt(BenchmarkRequest $req): BenchmarkResponse {
    $secret = "laravel-jwt-secret-2024"; // vuln-code-snippet vuln-line la_hardcoded_jwt
    $payload = json_encode(['user' => $req->post('username'), 'exp' => time() + 3600]);
    $token = base64_encode($payload) . '.' . hash_hmac('sha256', $payload, $secret);
    return BenchmarkResponse::json(['token' => $token]);
}
// vuln-code-snippet end la_hardcoded_jwt

// vuln-code-snippet start la_hardcoded_env
function la_hardcoded_env(BenchmarkRequest $req): BenchmarkResponse {
    $secret = getenv('JWT_SECRET'); // vuln-code-snippet safe-line la_hardcoded_env
    $payload = json_encode(['user' => $req->post('username'), 'exp' => time() + 3600]);
    $token = base64_encode($payload) . '.' . hash_hmac('sha256', $payload, $secret);
    return BenchmarkResponse::json(['token' => $token]);
}
// vuln-code-snippet end la_hardcoded_env

// vuln-code-snippet start la_weakrand_token
function la_weakrand_token(BenchmarkRequest $req): BenchmarkResponse {
    $token = md5(mt_rand()); // vuln-code-snippet vuln-line la_weakrand_token
    FakeDB::select("UPDATE users SET remember_token = ? WHERE id = ?", [$token, $req->post('user_id')]);
    return BenchmarkResponse::json(['remember_token' => $token]);
}
// vuln-code-snippet end la_weakrand_token

// vuln-code-snippet start la_weakrand_secure
function la_weakrand_secure(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32)); // vuln-code-snippet safe-line la_weakrand_secure
    FakeDB::select("UPDATE users SET remember_token = ? WHERE id = ?", [$token, $req->post('user_id')]);
    return BenchmarkResponse::json(['remember_token' => $token]);
}
// vuln-code-snippet end la_weakrand_secure

// vuln-code-snippet start la_csrf_no_middleware
function la_csrf_no_middleware(BenchmarkRequest $req): BenchmarkResponse {
    $newPassword = $req->post('new_password');
    $userId = $req->post('user_id');
    $hash = password_hash($newPassword, PASSWORD_BCRYPT);
    FakeDB::select("UPDATE users SET password = ? WHERE id = ?", [$hash, $userId]); // vuln-code-snippet vuln-line la_csrf_no_middleware
    return BenchmarkResponse::json(['password_changed' => true]);
}
// vuln-code-snippet end la_csrf_no_middleware

// vuln-code-snippet start la_csrf_middleware
function la_csrf_middleware(BenchmarkRequest $req): BenchmarkResponse {
    $sessionToken = $req->cookie('XSRF-TOKEN');
    $headerToken = $req->header('X-XSRF-TOKEN');
    if (!$sessionToken || !hash_equals($sessionToken, $headerToken)) { // vuln-code-snippet safe-line la_csrf_middleware
        return BenchmarkResponse::error('CSRF token mismatch', 419);
    }
    $newPassword = $req->post('new_password');
    $userId = $req->post('user_id');
    $hash = password_hash($newPassword, PASSWORD_BCRYPT);
    FakeDB::select("UPDATE users SET password = ? WHERE id = ?", [$hash, $userId]);
    return BenchmarkResponse::json(['password_changed' => true]);
}
// vuln-code-snippet end la_csrf_middleware
