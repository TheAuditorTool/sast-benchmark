require_relative 'shared'

def update_record(req)
  record_id = req.param('id')
  value = req.post('value')
  BenchmarkResponse.ok("record #{record_id} updated to #{value}")
end
