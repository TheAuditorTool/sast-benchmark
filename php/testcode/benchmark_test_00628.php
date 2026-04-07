<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00628(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('val');
    $html = "<script>var x = `$input`</script>";
    return BenchmarkResponse::html($html);
}
