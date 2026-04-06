<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_hardcoded_reflection
class KnownService016 {
    public function export(): string { return 'exported'; }
    public function report(): string { return 'reported'; }
}

function unsafereflect016(BenchmarkRequest $req): BenchmarkResponse {
    $method = $req->param('method');
    $ref = new ReflectionMethod('KnownService016', $method); // vuln-code-snippet safe-line php_reflect_hardcoded_reflection
    $obj = new KnownService016();
    $result = $ref->invoke($obj);
    return BenchmarkResponse::ok((string)$result);
}
// vuln-code-snippet end php_reflect_hardcoded_reflection
