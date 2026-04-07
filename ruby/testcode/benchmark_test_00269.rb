require_relative 'shared'

def find_employee_records(req)
  db = get_db_connection
  name = req.param('name')
  department = req.param('department', default: 'all')
  query = format("SELECT id, name, department, salary FROM employees WHERE name = '%s' AND department = '%s'", name, department)
  rows = db.execute(query)
  employees = rows.map { |r| { id: r[0], name: r[1], department: r[2] } }
  BenchmarkResponse.json({ employees: employees })
end
