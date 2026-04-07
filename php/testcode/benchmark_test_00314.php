<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00314(BenchmarkRequest $req): BenchmarkResponse {
    $arg = $req->param('arg');
    if (!ctype_alnum($arg)) {
        return BenchmarkResponse::badRequest('invalid arg');
    }
    $output = [];
    exec("lookup " . $arg, $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
