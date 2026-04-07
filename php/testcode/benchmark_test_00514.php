<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00514(BenchmarkRequest $req): BenchmarkResponse {
    $action = $req->post('action');
    $target = $req->post('target');
    performAdminAction($action, $target);
    return BenchmarkResponse::ok('action performed');
}
