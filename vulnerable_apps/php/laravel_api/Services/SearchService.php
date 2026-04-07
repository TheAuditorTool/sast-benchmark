<?php
require_once __DIR__ . '/../../../testcode/shared.php';

// vuln-code-snippet start la_sqli_search_raw
function la_sqli_search_raw(BenchmarkRequest $req): BenchmarkResponse {
    $term = $req->param('q');
    $sql = "SELECT * FROM tasks WHERE title LIKE '%" . $term . "%' OR description LIKE '%" . $term . "%'"; // vuln-code-snippet vuln-line la_sqli_search_raw
    $rows = FakeDB::selectRaw($sql);
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end la_sqli_search_raw

// vuln-code-snippet start la_sqli_search_bound
function la_sqli_search_bound(BenchmarkRequest $req): BenchmarkResponse {
    $term = $req->param('q');
    $pattern = '%' . $term . '%';
    $rows = FakeDB::select("SELECT * FROM tasks WHERE title LIKE ? OR description LIKE ?", [$pattern, $pattern]); // vuln-code-snippet safe-line la_sqli_search_bound
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end la_sqli_search_bound

// vuln-code-snippet start la_ssti_dynamic
function la_ssti_dynamic(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->param('template');
    $data = ['username' => 'admin', 'count' => 42];
    $rendered = preg_replace_callback('/\{\{\s*(\w+)\s*\}\}/', function ($m) use ($data) {
        return $data[$m[1]] ?? '';
    }, $template);
    eval('$output = "' . $rendered . '";'); // vuln-code-snippet vuln-line la_ssti_dynamic
    return BenchmarkResponse::html($output ?? '');
}
// vuln-code-snippet end la_ssti_dynamic

// vuln-code-snippet start la_ssti_file
function la_ssti_file(BenchmarkRequest $req): BenchmarkResponse {
    $data = ['username' => 'admin', 'count' => 42];
    $template = file_get_contents(__DIR__ . '/../templates/tasks.html');
    $rendered = preg_replace_callback('/\{\{\s*(\w+)\s*\}\}/', function ($m) use ($data) { // vuln-code-snippet safe-line la_ssti_file
        return htmlspecialchars($data[$m[1]] ?? '', ENT_QUOTES, 'UTF-8');
    }, $template);
    return BenchmarkResponse::html($rendered);
}
// vuln-code-snippet end la_ssti_file
