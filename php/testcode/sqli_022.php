<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_pdo_quote
function sqli022(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $safeId = $pdo->quote($id); // vuln-code-snippet safe-line php_sqli_pdo_quote
    $query = "SELECT * FROM users WHERE id = " . $safeId;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_pdo_quote
