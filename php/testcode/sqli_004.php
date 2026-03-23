<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_mysqli_prepare
function sqli004(BenchmarkRequest $req): BenchmarkResponse {
    $conn = getMysqliConnection();
    $id = $req->param('id');
    $stmt = $conn->prepare("SELECT * FROM users WHERE id = ?"); // vuln-code-snippet safe-line php_sqli_mysqli_prepare
    $stmt->bind_param("s", $id);
    $stmt->execute();
    $result = $stmt->get_result();
    $rows = $result->fetch_all(MYSQLI_ASSOC);
    $stmt->close();
    $conn->close();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_mysqli_prepare
