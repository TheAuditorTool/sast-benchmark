<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00665(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->postData;
    extract($data, EXTR_PREFIX_ALL, 'post');
    $name = $post_name ?? 'anonymous';
    return BenchmarkResponse::ok("hello $name");
}
