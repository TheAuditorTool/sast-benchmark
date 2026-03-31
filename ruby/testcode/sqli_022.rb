require_relative 'shared'

# vuln-code-snippet start ruby_sqli_sprintf_format
def find_employee_records(req)
  db = get_db_connection
  name = req.param('name')
  department = req.param('department', default: 'all')
  query = format("SELECT id, name, department, salary FROM employees WHERE name = '%s' AND department = '%s'", name, department)  # vuln-code-snippet vuln-line ruby_sqli_sprintf_format
  rows = db.execute(query)
  employees = rows.map { |r| { id: r[0], name: r[1], department: r[2] } }
  BenchmarkResponse.json({ employees: employees })
end
# vuln-code-snippet end ruby_sqli_sprintf_format
