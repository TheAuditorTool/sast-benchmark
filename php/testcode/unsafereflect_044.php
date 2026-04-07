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

// vuln-code-snippet start php_reflect_abstract_factory_validated
function unsafereflect044(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->param('type');
    $allowed = ['pdf', 'csv'];
    if (!in_array($type, $allowed, true)) return BenchmarkResponse::badRequest('invalid'); // vuln-code-snippet safe-line php_reflect_abstract_factory_validated
    $obj = HandlerFactory::create($type);
    return BenchmarkResponse::ok('created');
}
// vuln-code-snippet end php_reflect_abstract_factory_validated
