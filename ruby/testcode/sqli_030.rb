require_relative 'shared'

# vuln-code-snippet start ruby_sqli_find_by
def get_record_by_id(req)
  id = req.param('id').to_i
  return BenchmarkResponse.bad_request('invalid id') unless id > 0

  result = FakeActiveRecord::Base.find(id)  # vuln-code-snippet safe-line ruby_sqli_find_by
  record = result.to_a.first
  return BenchmarkResponse.bad_request('not found') unless record

  BenchmarkResponse.json({ record: record })
end
# vuln-code-snippet end ruby_sqli_find_by
