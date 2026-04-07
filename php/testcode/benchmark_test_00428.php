<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00428(BenchmarkRequest $req): BenchmarkResponse {
    $ctrl = $req->post('ctrl');
    include 'controllers/' . $ctrl . 'Controller.php';
    return BenchmarkResponse::ok('Controller loaded');
}
