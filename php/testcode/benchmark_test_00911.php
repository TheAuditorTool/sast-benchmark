<?php
require_once __DIR__ . '/shared.php';

class HandlerFactory {
    public static function create(string $type): object {
        return match($type) {
            'pdf' => new PdfHandler(),
            'csv' => new CsvHandler(),
        };
    }
}

function benchmarkTest00911(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->param('type');
    $allowed = ['pdf', 'csv'];
    if (!in_array($type, $allowed, true)) return BenchmarkResponse::badRequest('invalid');
    $obj = HandlerFactory::create($type);
    return BenchmarkResponse::ok('created');
}
