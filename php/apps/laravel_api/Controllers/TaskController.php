<?php
require_once __DIR__ . '/../../../testcode/shared.php';

// vuln-code-snippet start la_sqli_raw_query
function la_sqli_raw_query(BenchmarkRequest $req): BenchmarkResponse {
    $status = $req->param('status');
    $rows = FakeDB::selectRaw("SELECT * FROM tasks WHERE status = '" . $status . "'"); // vuln-code-snippet vuln-line la_sqli_raw_query
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end la_sqli_raw_query

// vuln-code-snippet start la_sqli_binding
function la_sqli_binding(BenchmarkRequest $req): BenchmarkResponse {
    $status = $req->param('status');
    $rows = FakeDB::select("SELECT * FROM tasks WHERE status = ?", [$status]); // vuln-code-snippet safe-line la_sqli_binding
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end la_sqli_binding

// vuln-code-snippet start la_sqli_whereraw
function la_sqli_whereraw(BenchmarkRequest $req): BenchmarkResponse {
    $filter = $req->param('filter');
    $sql = "SELECT * FROM tasks WHERE " . $filter; // vuln-code-snippet vuln-line la_sqli_whereraw
    $rows = FakeDB::selectRaw($sql);
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end la_sqli_whereraw

// vuln-code-snippet start la_sqli_where
function la_sqli_where(BenchmarkRequest $req): BenchmarkResponse {
    $status = $req->param('status');
    $where = ['status' => $status];
    $rows = FakeDB::select("SELECT * FROM tasks WHERE status = ?", array_values($where)); // vuln-code-snippet safe-line la_sqli_where
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end la_sqli_where

// vuln-code-snippet start la_massassign_create
function la_massassign_create(BenchmarkRequest $req): BenchmarkResponse {
    $task = $req->postData; // vuln-code-snippet vuln-line la_massassign_create
    FakeDB::select("INSERT INTO tasks (title, status, owner_id) VALUES (?, ?, ?)", [$task['title'] ?? '', $task['status'] ?? '', $task['owner_id'] ?? '']);
    return BenchmarkResponse::json(['created' => true]);
}
// vuln-code-snippet end la_massassign_create

// vuln-code-snippet start la_massassign_fillable
function la_massassign_fillable(BenchmarkRequest $req): BenchmarkResponse {
    $fillable = ['title', 'description'];
    $data = array_intersect_key($req->postData, array_flip($fillable)); // vuln-code-snippet safe-line la_massassign_fillable
    FakeDB::select("INSERT INTO tasks (title, description) VALUES (?, ?)", [$data['title'] ?? '', $data['description'] ?? '']);
    return BenchmarkResponse::json(['created' => true]);
}
// vuln-code-snippet end la_massassign_fillable

// vuln-code-snippet start la_massassign_update
function la_massassign_update(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    $fields = $req->postData; // vuln-code-snippet vuln-line la_massassign_update
    $sets = [];
    $vals = [];
    foreach ($fields as $col => $val) {
        $sets[] = "$col = ?";
        $vals[] = $val;
    }
    $vals[] = $id;
    FakeDB::select("UPDATE tasks SET " . implode(', ', $sets) . " WHERE id = ?", $vals);
    return BenchmarkResponse::json(['updated' => true]);
}
// vuln-code-snippet end la_massassign_update

// vuln-code-snippet start la_massassign_explicit
function la_massassign_explicit(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    $title = $req->post('title'); // vuln-code-snippet safe-line la_massassign_explicit
    FakeDB::select("UPDATE tasks SET title = ? WHERE id = ?", [$title, $id]);
    return BenchmarkResponse::json(['updated' => true]);
}
// vuln-code-snippet end la_massassign_explicit
