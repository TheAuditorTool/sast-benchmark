<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00902(BenchmarkRequest $req): BenchmarkResponse {
    define('TEMPLATE_VAR', 'color');
    ${'TEMPLATE_VAR'} = 'blue';
    return BenchmarkResponse::ok('set');
}
