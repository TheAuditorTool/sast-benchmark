<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_pdo_concat
function sqli001(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $query = "SELECT * FROM users WHERE id = " . $id; // vuln-code-snippet vuln-line php_sqli_pdo_concat
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_pdo_concat
