<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_like_prepare
function sqli016(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $search = $req->param('search');
    $stmt = $pdo->prepare("SELECT * FROM users WHERE name LIKE ?"); // vuln-code-snippet safe-line php_sqli_like_prepare
    $stmt->execute(['%' . $search . '%']);
    $rows = $stmt->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_like_prepare
