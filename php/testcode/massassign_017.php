<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_create_request_all
function massassign017(BenchmarkRequest $req): BenchmarkResponse {
    $data = $_POST;
    $model = new stdClass();
    foreach ($data as $k => $v) {
        $model->$k = $v; // vuln-code-snippet vuln-line php_massassign_create_request_all
    }
    $db = getDbConnection();
    $db->exec("INSERT INTO users DEFAULT VALUES");
    return BenchmarkResponse::ok('created');
}
// vuln-code-snippet end php_massassign_create_request_all
