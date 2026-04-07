<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00456(BenchmarkRequest $req): BenchmarkResponse {
    $suffix = intval($req->param('id'));
    $varName = 'field_' . $suffix;
    $$varName = $req->param('val');
    return BenchmarkResponse::ok('set');
}
