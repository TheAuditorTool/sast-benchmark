<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_from_array_unfiltered
function massassign023(BenchmarkRequest $req): BenchmarkResponse {
    $model = new stdClass();
    $postData = $_POST;
    array_walk($postData, function ($v, $k) use ($model) {
        $model->$k = $v; // vuln-code-snippet vuln-line php_massassign_from_array_unfiltered
    });
    return BenchmarkResponse::ok('stored');
}
// vuln-code-snippet end php_massassign_from_array_unfiltered
