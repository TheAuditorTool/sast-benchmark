<?php
require_once __DIR__ . '/shared.php';

class CacheFile014 {
    public string $path;
    public function __destruct() {
        if (file_exists($this->path)) {
            unlink($this->path);
        }
    }
}

function benchmarkTest00062(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('session');
    $obj = unserialize($data);
    return BenchmarkResponse::ok('Session loaded');
}
