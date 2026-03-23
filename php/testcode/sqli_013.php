<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_order_by_concat
function sqli013(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $sortCol = $req->param('sort');
    $query = "SELECT * FROM users ORDER BY " . $sortCol; // vuln-code-snippet vuln-line php_sqli_order_by_concat
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_order_by_concat
