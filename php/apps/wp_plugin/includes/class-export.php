<?php
// WordPress Events Manager Plugin - Data export / import
require_once __DIR__ . '/../../../testcode/shared.php';

// vuln-code-snippet start wp_cmdi_export
function wp_cmdi_export(BenchmarkRequest $req): BenchmarkResponse {
    $dbname = $req->param('dbname');
    $output = [];
    exec("mysqldump " . $dbname, $output); // vuln-code-snippet vuln-line wp_cmdi_export
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end wp_cmdi_export

// vuln-code-snippet start wp_cmdi_safe
function wp_cmdi_safe(BenchmarkRequest $req): BenchmarkResponse {
    $dbname = $req->param('dbname');
    $output = [];
    exec("mysqldump " . escapeshellarg($dbname), $output); // vuln-code-snippet safe-line wp_cmdi_safe
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end wp_cmdi_safe

// vuln-code-snippet start wp_pt_download
function wp_pt_download(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    $path = __DIR__ . '/../exports/' . $filename;
    $data = file_get_contents($path); // vuln-code-snippet vuln-line wp_pt_download
    return BenchmarkResponse::ok($data);
}
// vuln-code-snippet end wp_pt_download

// vuln-code-snippet start wp_pt_safe
function wp_pt_safe(BenchmarkRequest $req): BenchmarkResponse {
    $filename = basename($req->param('file'));
    $base = realpath(__DIR__ . '/../exports');
    $path = realpath($base . '/' . $filename);
    if ($path === false || strpos($path, $base) !== 0) { // vuln-code-snippet safe-line wp_pt_safe
        return BenchmarkResponse::badRequest('Invalid file');
    }
    $data = file_get_contents($path);
    return BenchmarkResponse::ok($data);
}
// vuln-code-snippet end wp_pt_safe

// vuln-code-snippet start wp_xxe_import
function wp_xxe_import(BenchmarkRequest $req): BenchmarkResponse {
    $xmlData = $req->bodyStr();
    $doc = simplexml_load_string($xmlData); // vuln-code-snippet vuln-line wp_xxe_import
    if ($doc === false) {
        return BenchmarkResponse::badRequest('Invalid XML');
    }
    return BenchmarkResponse::ok('Imported ' . count($doc->children()) . ' events');
}
// vuln-code-snippet end wp_xxe_import

// vuln-code-snippet start wp_xxe_safe
function wp_xxe_safe(BenchmarkRequest $req): BenchmarkResponse {
    $xmlData = $req->bodyStr();
    libxml_disable_entity_loader(true); // vuln-code-snippet safe-line wp_xxe_safe
    $doc = simplexml_load_string($xmlData, 'SimpleXMLElement', LIBXML_NONET);
    if ($doc === false) {
        return BenchmarkResponse::badRequest('Invalid XML');
    }
    return BenchmarkResponse::ok('Imported ' . count($doc->children()) . ' events');
}
// vuln-code-snippet end wp_xxe_safe

// vuln-code-snippet start wp_hardcoded_key
function wp_hardcoded_key(): string {
    $api_key = "wk_live_events_abc123"; // vuln-code-snippet vuln-line wp_hardcoded_key
    return $api_key;
}
// vuln-code-snippet end wp_hardcoded_key

// vuln-code-snippet start wp_hardcoded_option
function wp_hardcoded_option(): string {
    $api_key = getenv('EVENTS_API_KEY'); // vuln-code-snippet safe-line wp_hardcoded_option
    return $api_key ?: '';
}
// vuln-code-snippet end wp_hardcoded_option
