<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_param_array
function sqli028(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $ids = explode(',', $req->param('ids'));
    $placeholders = implode(',', array_fill(0, count($ids), '?')); // vuln-code-snippet safe-line php_sqli_param_array
    $stmt = $pdo->prepare("SELECT * FROM users WHERE id IN (" . $placeholders . ")");
    $stmt->execute($ids);
    $rows = $stmt->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_param_array
