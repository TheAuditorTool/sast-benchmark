<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_union_inject
function sqli027(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $query = "SELECT name, email FROM users WHERE id=" . $id; // vuln-code-snippet vuln-line php_sqli_union_inject
    $result = $pdo->query($query);
    $row = $result->fetch();
    return BenchmarkResponse::json($row ?: []);
}
// vuln-code-snippet end php_sqli_union_inject
