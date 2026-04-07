require_relative 'shared'

def delete_record(req)
  form_token = req.post('form_token')
  stored_token = req.cookie('form_token')
  return BenchmarkResponse.bad_request('request rejected') unless form_token && form_token == stored_token

  db = get_db_connection
  record_id = req.post('record_id')
  db.execute("DELETE FROM records WHERE id = ?", [record_id])
  BenchmarkResponse.ok('deleted')
end
