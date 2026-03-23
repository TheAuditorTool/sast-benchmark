<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_legacy_mysql
function sqli029(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    $conn = mysql_connect('localhost', 'root', '');
    mysql_select_db('benchmark', $conn);
    $result = mysql_query("SELECT * FROM users WHERE id=" . $id); // vuln-code-snippet vuln-line php_sqli_legacy_mysql
    $rows = [];
    while ($row = mysql_fetch_assoc($result)) {
        $rows[] = $row;
    }
    mysql_close($conn);
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_legacy_mysql
