<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_jsonschema_validated
function deserial042(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $data = json_decode($body, true);
    $schema = ['type' => 'object', 'properties' => ['id' => ['type' => 'integer']]];
    $valid = validateJsonSchema($data, $schema); // vuln-code-snippet safe-line php_deser_jsonschema_validated
    if (!$valid) {
        return BenchmarkResponse::badRequest('schema validation failed');
    }
    return BenchmarkResponse::json($data);
}
// vuln-code-snippet end php_deser_jsonschema_validated
