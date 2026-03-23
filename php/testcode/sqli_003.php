<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_mysqli_concat
function sqli003(BenchmarkRequest $req): BenchmarkResponse {
    $conn = getMysqliConnection();
    $id = $req->param('id');
    $result = mysqli_query($conn, "SELECT * FROM users WHERE id=" . $id); // vuln-code-snippet vuln-line php_sqli_mysqli_concat
    $rows = [];
    while ($row = mysqli_fetch_assoc($result)) {
        $rows[] = $row;
    }
    $conn->close();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_mysqli_concat
