<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_hidden_no_validate
function csrf010(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $pdo->prepare("UPDATE settings SET value = ? WHERE key = 'theme'")->execute([$req->post('theme')]); // vuln-code-snippet vuln-line php_csrf_hidden_no_validate
    return BenchmarkResponse::ok('Theme updated');
}
// vuln-code-snippet end php_csrf_hidden_no_validate
