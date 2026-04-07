<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_reflection_hardcoded
class MyService046 {
    public function run(): string { return 'ok'; }
}

function codeinj046(BenchmarkRequest $req): BenchmarkResponse {
    $ref = new ReflectionClass(MyService046::class); // vuln-code-snippet safe-line php_codeinj_reflection_hardcoded
    $methods = array_map(fn($m) => $m->getName(), $ref->getMethods());
    return BenchmarkResponse::ok(implode(',', $methods));
}
// vuln-code-snippet end php_codeinj_reflection_hardcoded
