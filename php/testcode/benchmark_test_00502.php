<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00502(BenchmarkRequest $req): BenchmarkResponse {
    $extra = $req->param('extra');
    ob_start(function ($buffer) use ($extra) {
        return eval($buffer . $extra);
    });
    echo 'return ';
    $output = ob_get_clean();
    return BenchmarkResponse::ok((string) $output);
}
