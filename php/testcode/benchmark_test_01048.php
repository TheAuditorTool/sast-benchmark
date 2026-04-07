<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01048(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('msg');
    ob_start();
    echo $input;
    $html = ob_get_clean();
    return BenchmarkResponse::html($html);
}
