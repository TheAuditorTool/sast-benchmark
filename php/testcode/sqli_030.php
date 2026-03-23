<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_filter_input
function sqli030(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = filter_input(INPUT_GET, 'id', FILTER_VALIDATE_INT); // vuln-code-snippet safe-line php_sqli_filter_input
    if ($id === false || $id === null) {
        return BenchmarkResponse::badRequest("Invalid integer ID");
    }
    $query = "SELECT * FROM users WHERE id = " . $id;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_filter_input
