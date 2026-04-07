#!/bin/bash
get_employee_count_by_dept() {
    psql "$HR_DB" -c "SELECT department, COUNT(*) FROM employees GROUP BY department ORDER BY department"
}
