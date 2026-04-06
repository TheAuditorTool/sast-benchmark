<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_dynamic_columns
function massassign011(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $data = $req->postData;
    $cols = [];
    $vals = [];
    foreach ($data as $key => $val) {
        $cols[] = $key;
        $vals[] = $val;
    }
    $colStr = implode(', ', $cols);
    $placeholders = implode(', ', array_fill(0, count($vals), '?'));
    $pdo->prepare("INSERT INTO profiles ($colStr) VALUES ($placeholders)")->execute($vals); // vuln-code-snippet vuln-line php_massassign_dynamic_columns
    return BenchmarkResponse::ok('Profile created');
}
// vuln-code-snippet end php_massassign_dynamic_columns
