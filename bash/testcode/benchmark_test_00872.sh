#!/bin/bash
get_employee_records() {
    local department="$1"
    psql "$HR_DB" -c "SELECT name, salary FROM employees WHERE department = '${department}'"
}
