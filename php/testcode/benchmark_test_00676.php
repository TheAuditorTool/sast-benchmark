<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00676(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('hash');
    if (sha1($input) == '0e89100756225895e68e7ba9189a28f18fc8bcc5') {
        return BenchmarkResponse::ok('authenticated');
    }
    return BenchmarkResponse::badRequest('denied');
}
