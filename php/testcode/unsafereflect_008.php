<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_dynamic_method
class Processor008 {
    public function export(): string { return 'exported'; }
    public function delete(): string { return 'deleted'; }
}

function unsafereflect008(BenchmarkRequest $req): BenchmarkResponse {
    $method = $req->param('action');
    $obj = new Processor008();
    $result = $obj->$method(); // vuln-code-snippet vuln-line php_reflect_dynamic_method
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_reflect_dynamic_method
