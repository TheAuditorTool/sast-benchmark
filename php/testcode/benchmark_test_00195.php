<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00195(BenchmarkRequest $req): BenchmarkResponse {
    $resourceId = (int) $req->param('id');
    deleteResource($resourceId);
    return BenchmarkResponse::ok('resource deleted');
}
