<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_update_json_unfiltered
function massassign021(BenchmarkRequest $req): BenchmarkResponse {
    $body = json_decode($req->bodyStr(), true);
    $model = new stdClass();
    foreach ($body as $k => $v) {
        $model->$k = $v; // vuln-code-snippet vuln-line php_massassign_update_json_unfiltered
    }
    return BenchmarkResponse::ok('updated');
}
// vuln-code-snippet end php_massassign_update_json_unfiltered
