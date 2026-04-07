<?php
require_once __DIR__ . '/shared.php';

class Template {
    const VAR_NAME = 'title';
}

function benchmarkTest00706(BenchmarkRequest $req): BenchmarkResponse {
    $val = $req->param('val');
    ${Template::VAR_NAME} = $val;
    return BenchmarkResponse::ok('set');
}
