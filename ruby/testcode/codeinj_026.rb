require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_rescue_eval
def process_request(req)
  user_input = req.param('payload')
  begin
    Integer(user_input)
  rescue ArgumentError => e
    eval(e.message) # vuln-code-snippet vuln-line ruby_codeinj_rescue_eval
  end
  BenchmarkResponse.json({ status: 'processed' })
end
# vuln-code-snippet end ruby_codeinj_rescue_eval
