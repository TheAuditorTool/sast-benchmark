<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00556(BenchmarkRequest $req): BenchmarkResponse {
    $source = $req->param('source_class');
    class_alias($source, 'AliasedHandler');
    $handler = new AliasedHandler();
    return BenchmarkResponse::ok($handler->run());
}
