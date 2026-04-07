require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_string_interp_eval
def compute_value(req)
  val = req.param('val')
  result = eval("result = #{val}") # vuln-code-snippet vuln-line ruby_codeinj_string_interp_eval
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_codeinj_string_interp_eval
