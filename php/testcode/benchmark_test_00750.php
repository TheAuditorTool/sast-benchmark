<?php
require_once __DIR__ . '/shared.php';

class UserService {
    public function __construct(string $data = '') {}
}

function benchmarkTest00750(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('input');
    $svc = new UserService($data);
    return BenchmarkResponse::ok('created');
}
