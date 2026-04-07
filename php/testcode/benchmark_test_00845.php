<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00845(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('msg');
    ob_start();
    echo htmlspecialchars($input, ENT_QUOTES);
    $out = ob_get_clean();
    return BenchmarkResponse::ok($out);
}
