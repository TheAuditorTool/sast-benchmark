<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00668(BenchmarkRequest $req): BenchmarkResponse {
    $tpl = $req->param('tpl');
    if (!in_array($tpl, ['home', 'about', 'contact'], true)) {
        return BenchmarkResponse::badRequest('Invalid template');
    }
    include __DIR__ . "/pages/{$tpl}.php";
    return BenchmarkResponse::ok('Rendered');
}
