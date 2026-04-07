<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00602(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('data');
    $safe = json_encode($input, JSON_HEX_TAG | JSON_HEX_AMP | JSON_HEX_APOS | JSON_HEX_QUOT);
    $html = "<script>var data = $safe</script>";
    return BenchmarkResponse::html($html);
}
