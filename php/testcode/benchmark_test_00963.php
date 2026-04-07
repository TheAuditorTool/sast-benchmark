<?php
require_once __DIR__ . '/shared.php';

function formatOutput(string $name, string $email): string {
    return $name . ' <' . $email . '>';
}

function benchmarkTest00963(BenchmarkRequest $req): BenchmarkResponse {
    $result = formatOutput018(
        name: $req->param('name'),
        email: $req->param('email'),
    );
    return BenchmarkResponse::ok($result);
}
