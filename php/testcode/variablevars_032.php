<?php
require_once __DIR__ . '/shared.php';

class SomeClass {
    public static function staticMethod(): string { return 'called'; }
}

// vuln-code-snippet start php_vv_class_alias_dynamic
function variablevars032(BenchmarkRequest $req): BenchmarkResponse {
    $cls = $req->post('cls');
    $$cls = 'SomeClass';
    $$cls::staticMethod(); // vuln-code-snippet vuln-line php_vv_class_alias_dynamic
    return BenchmarkResponse::ok('called');
}
// vuln-code-snippet end php_vv_class_alias_dynamic
