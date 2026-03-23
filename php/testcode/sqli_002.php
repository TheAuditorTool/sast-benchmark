<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_pdo_prepare
function sqli002(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $stmt = $pdo->prepare("SELECT * FROM users WHERE id = ?"); // vuln-code-snippet safe-line php_sqli_pdo_prepare
    $stmt->execute([$id]);
    $rows = $stmt->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_pdo_prepare
