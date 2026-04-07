require_relative 'shared'

def get_record_by_id(req)
  id = req.param('id').to_i
  return BenchmarkResponse.bad_request('invalid id') unless id > 0

  result = FakeActiveRecord::Base.find(id)
  record = result.to_a.first
  return BenchmarkResponse.bad_request('not found') unless record

  BenchmarkResponse.json({ record: record })
end
