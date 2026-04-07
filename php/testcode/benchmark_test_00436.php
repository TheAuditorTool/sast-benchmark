<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00436(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->post('template');
    ob_start();
    eval("?>" . $template);
    $output = ob_get_clean();
    return BenchmarkResponse::html($output);
}
