require_relative 'shared'

DATA = {
  'name'    => 'Alice',
  'country' => 'US',
  'plan'    => 'free'
}.freeze

# vuln-code-snippet start ruby_codeinj_no_eval_path
def lookup_field(req)
  key = req.param('key')
  value = DATA[key] || 'unknown' # vuln-code-snippet safe-line ruby_codeinj_no_eval_path
  BenchmarkResponse.json({ key: key, value: value })
end
# vuln-code-snippet end ruby_codeinj_no_eval_path
