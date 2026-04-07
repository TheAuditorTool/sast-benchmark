<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_bearer_no_cookie
function csrf035(BenchmarkRequest $req): BenchmarkResponse {
    $authHeader = $req->header('Authorization');
    if (!str_starts_with($authHeader, 'Bearer ')) {
        return BenchmarkResponse::error('unauthorized');
    }
    $token = substr($authHeader, 7);
    $user = validateBearerToken($token); // vuln-code-snippet safe-line php_csrf_bearer_no_cookie
    if (!$user) {
        return BenchmarkResponse::error('invalid token');
    }
    return BenchmarkResponse::json(fetchUserData($user['id']));
}
// vuln-code-snippet end php_csrf_bearer_no_cookie
