<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01246(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('handler');
    $allowed = ['JsonHandler', 'XmlHandler', 'CsvHandler'];
    if (!in_array($className, $allowed, true)) {
        return BenchmarkResponse::badRequest('unknown handler');
    }
    $obj = new $className();
    $result = $obj->process($req->bodyStr());
    return BenchmarkResponse::ok($result);
}
