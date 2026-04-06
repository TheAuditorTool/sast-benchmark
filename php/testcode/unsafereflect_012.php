<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_class_alias
function unsafereflect012(BenchmarkRequest $req): BenchmarkResponse {
    $source = $req->param('source_class');
    class_alias($source, 'AliasedHandler'); // vuln-code-snippet vuln-line php_reflect_class_alias
    $handler = new AliasedHandler();
    return BenchmarkResponse::ok($handler->run());
}
// vuln-code-snippet end php_reflect_class_alias
