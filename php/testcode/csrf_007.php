<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_referer_only
function csrf007(BenchmarkRequest $req): BenchmarkResponse {
    $referer = $req->header('Referer');
    if (empty($referer)) { // vuln-code-snippet vuln-line php_csrf_referer_only
        return BenchmarkResponse::badRequest('Missing referer');
    }
    $pdo = getDbConnection();
    $pdo->prepare("UPDATE users SET email = ? WHERE id = ?")->execute([$req->post('email'), $req->post('id')]);
    return BenchmarkResponse::ok('Email updated');
}
// vuln-code-snippet end php_csrf_referer_only
