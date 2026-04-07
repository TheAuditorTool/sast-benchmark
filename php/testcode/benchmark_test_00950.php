<?php
require_once __DIR__ . '/shared.php';

class BaseHandler009 {
    public static function execute(): string { return 'executed'; }
}

function benchmarkTest00950(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('handler');
    $result = forward_static_call([$className, 'execute']);
    return BenchmarkResponse::ok((string)$result);
}
