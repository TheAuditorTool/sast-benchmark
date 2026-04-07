require_relative 'shared'

def stateless_api_action(req)
  api_key = req.header('X-API-Key')
  return BenchmarkResponse.bad_request('missing API key') unless api_key
  db = get_db_connection
  key_record = db.execute("SELECT user_id FROM api_keys WHERE key_hash = ?", [Digest::SHA256.hexdigest(api_key)]).first
  return BenchmarkResponse.bad_request('invalid key') unless key_record
  BenchmarkResponse.json({ result: key_record[0] })
end
