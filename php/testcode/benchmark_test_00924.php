<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00924(BenchmarkRequest $req): BenchmarkResponse {
    $name  = htmlspecialchars($req->param('name'));
    $email = htmlspecialchars($req->param('email'));
    $out = compact('name', 'email');
    return BenchmarkResponse::json($out);
}
