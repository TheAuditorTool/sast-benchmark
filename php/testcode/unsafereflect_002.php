<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_factory_map
function unsafereflect002(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->param('type');
    $map = [
        'json' => 'JsonHandler',
        'xml'  => 'XmlHandler',
        'csv'  => 'CsvHandler',
    ];
    $className = $map[$type] ?? 'DefaultHandler'; // vuln-code-snippet safe-line php_reflect_factory_map
    $obj = new $className();
    return BenchmarkResponse::ok($obj->handle());
}
// vuln-code-snippet end php_reflect_factory_map
