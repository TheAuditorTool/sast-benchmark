<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_destruct_file_write
class FileWriter022 {
    public string $path = '';
    public string $data = '';
    public function __destruct() {
        file_put_contents($this->path, $this->data);
    }
}

function deserial022(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->post('data');
    $obj = unserialize($input); // vuln-code-snippet vuln-line php_deser_destruct_file_write
    return BenchmarkResponse::ok('done');
}
// vuln-code-snippet end php_deser_destruct_file_write
