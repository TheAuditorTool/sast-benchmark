<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00548(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('val');
    $html = "<data>" . htmlspecialchars($input, ENT_XML1, 'UTF-8') . "</data>";
    return BenchmarkResponse::html($html);
}
