<?php
require_once __DIR__ . '/shared.php';

class FileWriter022 {
    public string $path = '';
    public string $data = '';
    public function __destruct() {
        file_put_contents($this->path, $this->data);
    }
}

function benchmarkTest00595(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->post('data');
    $obj = unserialize($input);
    return BenchmarkResponse::ok('done');
}
