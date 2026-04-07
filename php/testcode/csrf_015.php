<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_ajax_json_no_check
function csrf015(BenchmarkRequest $req): BenchmarkResponse {
    $body = json_decode($req->bodyStr(), true);
    $userId = (int) ($body['user_id'] ?? 0);
    $role = $body['role'] ?? 'viewer';
    updateUserRole($userId, $role); // vuln-code-snippet vuln-line php_csrf_ajax_json_no_check
    return BenchmarkResponse::json(['updated' => true]);
}
// vuln-code-snippet end php_csrf_ajax_json_no_check
