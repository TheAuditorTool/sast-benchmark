require_relative 'shared'

module Oj
  def self.load(input, opts = {}); input; end
end

# vuln-code-snippet start ruby_deser_oj_strict
def parse_oj_safe(req)
  data = Oj.load(req.body_str, mode: :strict) # vuln-code-snippet safe-line ruby_deser_oj_strict
  BenchmarkResponse.ok(data.to_s)
end
# vuln-code-snippet end ruby_deser_oj_strict
