<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_intval_cast
function sqli012(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = intval($req->param('id')); // vuln-code-snippet safe-line php_sqli_intval_cast
    $query = "SELECT * FROM users WHERE id = " . $id;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_intval_cast
