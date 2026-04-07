<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_usort_eval
function codeinj026(BenchmarkRequest $req): BenchmarkResponse {
    $direction = $req->param('direction');
    $arr = [3, 1, 4, 1, 5, 9, 2, 6];
    $comparatorCode = 'return ' . $direction . ';';
    eval('$cmp = function($a, $b) { ' . $comparatorCode . ' };'); // vuln-code-snippet vuln-line php_codeinj_usort_eval
    usort($arr, $cmp);
    return BenchmarkResponse::ok(implode(',', $arr));
}
// vuln-code-snippet end php_codeinj_usort_eval
