require_relative 'shared'

def update_resource_put(req)
  resource_id = req.param('id').to_i
  data = req.post('data')
  db = get_db_connection
  db.execute("UPDATE resources SET data = ? WHERE id = ?", [data, resource_id])
  BenchmarkResponse.json({ result: 'updated' })
end
