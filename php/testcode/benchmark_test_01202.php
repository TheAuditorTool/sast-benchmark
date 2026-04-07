<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01202(BenchmarkRequest $req): BenchmarkResponse {
    $doc = $req->file('document');
    $target = '/srv/docs/' . $doc['name'];
    move_uploaded_file($doc['tmp_name'], $target);
    return BenchmarkResponse::ok('document saved');
}
