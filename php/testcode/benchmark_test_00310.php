<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00310(BenchmarkRequest $req): BenchmarkResponse {
    $cmd = new stdClass();
    $cmd->name = $_POST['name'] ?? '';
    $cmd->email = $_POST['email'] ?? '';
    return BenchmarkResponse::ok('command dispatched');
}
