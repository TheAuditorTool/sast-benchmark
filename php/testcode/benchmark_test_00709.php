<?php
require_once __DIR__ . '/shared.php';

class FileReader030 {
    public string $file = '';
    public function __toString(): string {
        return file_get_contents($this->file);
    }
}

function benchmarkTest00709(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $obj = unserialize($data);
    $output = (string) $obj;
    return BenchmarkResponse::ok($output);
}
