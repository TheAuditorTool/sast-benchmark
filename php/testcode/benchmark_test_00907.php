<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00907(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $data = json_decode($body, true);
    $schema = ['type' => 'object', 'properties' => ['id' => ['type' => 'integer']]];
    $valid = validateJsonSchema($data, $schema);
    if (!$valid) {
        return BenchmarkResponse::badRequest('schema validation failed');
    }
    return BenchmarkResponse::json($data);
}
