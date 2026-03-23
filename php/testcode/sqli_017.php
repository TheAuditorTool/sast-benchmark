<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_multihop_taint
function buildQuery(string $id): string {
    return "SELECT * FROM users WHERE id = " . $id;
}

function sqli017(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $query = buildQuery($id);
    $result = $pdo->query($query); // vuln-code-snippet vuln-line php_sqli_multihop_taint
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_multihop_taint
