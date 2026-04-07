<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00277(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->param('type');
    $map = [
        'json' => 'JsonHandler',
        'xml'  => 'XmlHandler',
        'csv'  => 'CsvHandler',
    ];
    $className = $map[$type] ?? 'DefaultHandler';
    $obj = new $className();
    return BenchmarkResponse::ok($obj->handle());
}
