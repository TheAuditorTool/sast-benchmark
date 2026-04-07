<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_dynamic_columns_insert
function massassign022(BenchmarkRequest $req): BenchmarkResponse {
    $db = getDbConnection();
    $cols = implode(',', array_keys($_POST));
    $vals = implode(',', array_fill(0, count($_POST), '?'));
    $db->prepare("INSERT INTO users ($cols) VALUES ($vals)")->execute(array_values($_POST)); // vuln-code-snippet vuln-line php_massassign_dynamic_columns_insert
    return BenchmarkResponse::ok('inserted');
}
// vuln-code-snippet end php_massassign_dynamic_columns_insert
