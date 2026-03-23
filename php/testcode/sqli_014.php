<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_order_by_whitelist
function sqli014(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $sortCol = $req->param('sort');
    $allowed = ['id', 'name', 'email', 'created_at'];
    $safeSort = in_array($sortCol, $allowed, true) ? $sortCol : 'id'; // vuln-code-snippet safe-line php_sqli_order_by_whitelist
    $query = "SELECT * FROM users ORDER BY " . $safeSort;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_order_by_whitelist
