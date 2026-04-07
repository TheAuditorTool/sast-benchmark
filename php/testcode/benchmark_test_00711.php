<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00711(BenchmarkRequest $req): BenchmarkResponse {
    $ext = pathinfo($req->param('name'), PATHINFO_EXTENSION);
    $safeName = bin2hex(random_bytes(16)) . '.' . strtolower($ext);
    file_put_contents('/var/app/uploads/' . $safeName, $req->bodyStr());
    return BenchmarkResponse::ok($safeName);
}
