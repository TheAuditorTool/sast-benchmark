require_relative 'shared'

def delete_record(req)
  db = get_db_connection
  record_id = req.post('record_id')
  db.execute("DELETE FROM records WHERE id = ?", [record_id])
  BenchmarkResponse.ok('deleted')
end
