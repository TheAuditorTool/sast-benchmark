<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00631(BenchmarkRequest $req): BenchmarkResponse {
    $tpl  = basename($req->param('tpl'));
    include TEMPLATES_DIR . $tpl . '.php';
    return BenchmarkResponse::ok('Rendered');
}
