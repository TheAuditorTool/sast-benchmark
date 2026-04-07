<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00220(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('import');
    $content = file_get_contents($file['tmp_name']);
    $data = unserialize($content);
    return BenchmarkResponse::json(['imported' => count($data)]);
}
