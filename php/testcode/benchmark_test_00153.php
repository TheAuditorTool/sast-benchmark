<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00153(BenchmarkRequest $req): BenchmarkResponse {
    $tpl = $req->param('tpl');
    ob_start();
    include 'php://input';
    $output = ob_get_clean();
    return BenchmarkResponse::ok($output);
}
