<?php
require_once __DIR__ . '/shared.php';

class SomeClass {
    public static function staticMethod(): string { return 'called'; }
}

function benchmarkTest00207(BenchmarkRequest $req): BenchmarkResponse {
    $cls = $req->post('cls');
    $$cls = 'SomeClass';
    $$cls::staticMethod();
    return BenchmarkResponse::ok('called');
}
