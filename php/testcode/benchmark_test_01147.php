<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01147(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $dept = $req->param('department');
    $minSalary = $req->param('min_salary');
    $sql = "SELECT name, salary FROM employees WHERE department = '$dept' AND salary >= $minSalary";
    return BenchmarkResponse::json($pdo->query($sql)->fetchAll());
}
