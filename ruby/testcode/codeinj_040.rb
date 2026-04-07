require_relative 'shared'

TRANSFORMS = {
  'upcase'  => ->(s) { s.upcase },
  'reverse' => ->(s) { s.reverse },
  'strip'   => ->(s) { s.strip },
  'chomp'   => ->(s) { s.chomp }
}.freeze

# vuln-code-snippet start ruby_codeinj_hardcoded_transform
def apply_transform(req)
  fn = req.param('fn')
  input = req.param('input')
  transform = TRANSFORMS.fetch(fn) { return BenchmarkResponse.bad_request('unknown transform') }
  result = transform.call(input) # vuln-code-snippet safe-line ruby_codeinj_hardcoded_transform
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_codeinj_hardcoded_transform
