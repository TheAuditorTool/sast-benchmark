<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_eloquent_where
function sqli020(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $status = $req->param('status');
    $stmt = $pdo->prepare("SELECT * FROM orders WHERE status = ?"); // vuln-code-snippet safe-line php_sqli_eloquent_where
    $stmt->execute([$status]);
    $rows = $stmt->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_eloquent_where
